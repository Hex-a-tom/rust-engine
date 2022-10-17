extern crate vulkano;
extern crate vulkano_win;
extern crate winit;
extern crate alloc;

use std::sync::Arc;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};

use vulkano::{instance::{Instance, InstanceCreateInfo, InstanceExtensions, Version}, library};
use vulkano::instance::debug::{DebugUtilsMessenger, DebugUtilsMessengerCreateInfo};
use vulkano::library::VulkanLibrary;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_LUNARG_standard_validation"
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

#[allow(unused)]
struct HelloTriangleApplication {
    instance: Arc<Instance>,
    debug_callback: Option<DebugUtilsMessenger>,
    // events_loop: EventLoop<()>,
    // window: Window,
}

impl HelloTriangleApplication {
    pub fn initialize() -> Self {
        let instance = Self::create_instance();
        let debug_callback = Self::setup_debug_callback(&instance);
        // let events_loop = Self::init_events_loop();
        // let window = Self::init_window(&events_loop);

        Self {
            instance,
            debug_callback,
            // events_loop,
            // window,
        }
    }

    fn _init_events_loop() -> EventLoop<()> {
        EventLoop::new()
    }

    fn _init_window(events_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new()
            .with_title("Vulkan")
            .with_inner_size(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop).expect("Failed to create window")
    }

    fn create_instance() -> Arc<Instance> {
        let library = VulkanLibrary::new()
            .expect("Failed to load default vulkan library");

        if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layer_support(&library){
            println!("Validation layers requested, but not available!")
        }

        let required_extensions = Self::get_required_extensions(&library);

        if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support(&library){
            Instance::new(
                library, 
                InstanceCreateInfo {
                    application_name: Some(env!("CARGO_PKG_NAME").to_owned()),
                    application_version: Version {
                        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
                    },
                    enabled_extensions: required_extensions,
                    enabled_layers: VALIDATION_LAYERS.iter().map(|&x| x.into()).collect(),
                    ..Default::default()
                }
                )
                .expect("Failed to create vulkan instance")
        } else {   
            Instance::new(
                library, 
                InstanceCreateInfo {
                    application_name: Some(env!("CARGO_PKG_NAME").to_owned()),
                    application_version: Version {
                        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
                    },
                    enabled_extensions: required_extensions,
                    ..Default::default()
                }
                )
                .expect("Failed to create vulkan instance")
        }


    }

    fn check_validation_layer_support(library: &VulkanLibrary) -> bool {
        let layers: Vec<_> = library
            .layer_properties().unwrap()
            .map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn get_required_extensions(library: &VulkanLibrary) -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions(library);
        if ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }

        extensions
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugUtilsMessenger> {
        if !ENABLE_VALIDATION_LAYERS  {
            return None;
        }

        unsafe {
            DebugUtilsMessenger::new(
                instance.to_owned(),
                DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg| {
                    println!("Debug callback: {:?}", msg.description);
                })),
            ).ok()
        }
    }

    #[allow(unused)]
    fn main_loop(&mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(
                    f64::from(WIDTH),
                    f64::from(HEIGHT))
                )
            .build(&event_loop)
            .expect("Failed to create window");

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id
                } if window_id == window.id() => {
                    control_flow.set_exit();
            }
                _ => {}
            }
        });
    }
}

fn main() {
    let mut app = HelloTriangleApplication::initialize();
    app.main_loop();
}
