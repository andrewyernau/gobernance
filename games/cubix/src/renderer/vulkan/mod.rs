#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod app;
mod commands;
mod device;
mod instance;
mod pipeline;
mod swapchain;
mod sync;
mod types;

pub use app::Renderer;

pub(crate) use commands::{create_command_buffers, create_command_pool, create_framebuffers};
pub(crate) use device::{QueueFamilyIndices, create_logical_device, pick_physical_device};
pub(crate) use instance::create_instance;
pub(crate) use pipeline::{create_pipeline, create_render_pass};
pub(crate) use swapchain::{SwapchainSupport, create_swapchain, create_swapchain_image_views};
pub(crate) use sync::{create_swapchain_sync_objects, create_sync_objects};
pub(crate) use types::{
    AppData, DEVICE_EXTENSIONS, MAX_FRAMES_IN_FLIGHT, PORTABILITY_MACOS_VERSION, SuitabilityError,
    VALIDATION_ENABLED, VALIDATION_LAYER,
};
