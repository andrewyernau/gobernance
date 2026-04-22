use thiserror::Error;
use vulkanalia::Version;
use vulkanalia::prelude::v1_0::*;

pub(crate) const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub(crate) const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
pub(crate) const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
pub(crate) const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub(crate) struct SuitabilityError(pub(crate) &'static str);

#[derive(Clone, Debug, Default)]
pub(crate) struct AppData {
    pub(crate) messenger: vk::DebugUtilsMessengerEXT,
    pub(crate) surface: vk::SurfaceKHR,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) graphics_queue: vk::Queue,
    pub(crate) present_queue: vk::Queue,
    pub(crate) swapchain_format: vk::Format,
    pub(crate) swapchain_extent: vk::Extent2D,
    pub(crate) swapchain: vk::SwapchainKHR,
    pub(crate) swapchain_images: Vec<vk::Image>,
    pub(crate) swapchain_image_views: Vec<vk::ImageView>,
    pub(crate) render_pass: vk::RenderPass,
    pub(crate) pipeline_layout: vk::PipelineLayout,
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) framebuffers: Vec<vk::Framebuffer>,
    pub(crate) command_pool: vk::CommandPool,
    pub(crate) command_buffers: Vec<vk::CommandBuffer>,
    pub(crate) in_flight_fences: Vec<vk::Fence>,
    pub(crate) images_in_flight: Vec<vk::Fence>,
    pub(crate) image_available_semaphores: Vec<vk::Semaphore>,
    pub(crate) render_finished_semaphores: Vec<vk::Semaphore>,
}
