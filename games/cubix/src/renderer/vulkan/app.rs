use anyhow::{Result, anyhow};
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{
    ExtDebugUtilsExtensionInstanceCommands, KhrSurfaceExtensionInstanceCommands,
    KhrSwapchainExtensionDeviceCommands,
};
use vulkanalia::window as vk_window;
use winit::window::Window;

use super::{
    AppData, MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED, create_command_buffers, create_command_pool,
    create_framebuffers, create_instance, create_logical_device, create_pipeline,
    create_render_pass, create_swapchain, create_swapchain_image_views,
    create_swapchain_sync_objects, create_sync_objects, pick_physical_device,
};

#[derive(Clone, Debug)]
pub struct Renderer {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
    frame: usize,
}

impl Renderer {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|builder| anyhow!("{}", builder))?;
        let mut data = AppData::default();

        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, window, window)?;
        pick_physical_device(&instance, &mut data)?;

        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_command_pool(&instance, &device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;

        Ok(Self {
            entry,
            instance,
            data,
            device,
            frame: 0,
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        let image_index = match self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::MAX,
            self.data.image_available_semaphores[self.frame],
            vk::Fence::null(),
        ) {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => {
                self.recreate_swapchain(window)?;
                return Ok(());
            }
            Err(error) => return Err(anyhow!(error)),
        };

        let image_in_flight = self.data.images_in_flight[image_index];
        if !image_in_flight.is_null() {
            self.device
                .wait_for_fences(&[image_in_flight], true, u64::MAX)?;
        }

        self.data.images_in_flight[image_index] = in_flight_fence;

        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index]];
        let signal_semaphores = &[self.data.render_finished_semaphores[image_index]];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.reset_fences(&[in_flight_fence])?;
        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let swapchain_changed = match self
            .device
            .queue_present_khr(self.data.present_queue, &present_info)
        {
            Ok(vk::SuccessCode::SUBOPTIMAL_KHR) => true,
            Ok(_) => false,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => true,
            Err(error) => return Err(anyhow!(error)),
        };

        if swapchain_changed {
            self.recreate_swapchain(window)?;
        }

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;
        Ok(())
    }

    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }

        self.device.device_wait_idle()?;
        self.destroy_swapchain();

        create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;
        create_render_pass(&self.device, &mut self.data)?;
        create_pipeline(&self.device, &mut self.data)?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_command_buffers(&self.device, &mut self.data)?;
        create_swapchain_sync_objects(&self.device, &mut self.data)?;

        self.data
            .images_in_flight
            .resize(self.data.swapchain_images.len(), vk::Fence::null());

        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) {
        self.data
            .render_finished_semaphores
            .iter()
            .for_each(|semaphore| self.device.destroy_semaphore(*semaphore, None));
        self.data.render_finished_semaphores.clear();
        self.data.images_in_flight.clear();

        if !self.data.command_buffers.is_empty() {
            self.device
                .free_command_buffers(self.data.command_pool, &self.data.command_buffers);
            self.data.command_buffers.clear();
        }

        self.data
            .framebuffers
            .iter()
            .for_each(|framebuffer| self.device.destroy_framebuffer(*framebuffer, None));
        self.data.framebuffers.clear();

        self.device.destroy_pipeline(self.data.pipeline, None);
        self.data.pipeline = vk::Pipeline::null();

        self.device
            .destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.data.pipeline_layout = vk::PipelineLayout::null();

        self.device.destroy_render_pass(self.data.render_pass, None);
        self.data.render_pass = vk::RenderPass::null();

        self.data
            .swapchain_image_views
            .iter()
            .for_each(|image_view| self.device.destroy_image_view(*image_view, None));
        self.data.swapchain_image_views.clear();
        self.data.swapchain_images.clear();

        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.data.swapchain = vk::SwapchainKHR::null();
    }

    pub unsafe fn destroy(&mut self) {
        let _ = &self.entry;
        self.device.device_wait_idle().unwrap();

        self.data
            .in_flight_fences
            .iter()
            .for_each(|fence| self.device.destroy_fence(*fence, None));
        self.data
            .image_available_semaphores
            .iter()
            .for_each(|semaphore| self.device.destroy_semaphore(*semaphore, None));

        self.destroy_swapchain();
        self.device
            .destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_device(None);

        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }
}
