use winit::event_loop::{EventLoop, ControlFlow};
use winit::dpi::{LogicalSize, PhysicalSize};
use gfx_hal::window::Extent2D;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};

const APP_NAME : &'static str = "Test Triangle";
const WINDOW_SIZE: [u32; 2] = [512, 512];

fn main() {

    let event_loop = EventLoop::new();

    let (virtual_size, real_size) = {
        let scale_factor = event_loop.primary_monitor().scale_factor();
        let unscaled_size : LogicalSize<u32> = WINDOW_SIZE.into();
        let scaled_size : PhysicalSize<u32> = unscaled_size.to_physical(scale_factor);

        (unscaled_size, scaled_size)
    };

    let mut surface_extent = Extent2D {
        width : real_size.width,
        height: real_size.height
    };

    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(virtual_size)
        .build(&event_loop)
        .expect("Failed to create window");

    let mut should_configure_swap_chain = true;

    event_loop.run(move|event, _, control_flow| {
       match event {
           Event::NewEvents(_) => {}
           Event::WindowEvent { event, .. } => match event {
               WindowEvent::Resized(dims) => {
                   surface_extent = Extent2D {
                       width: dims.width,
                       height: dims.height,
                   };
                   should_configure_swap_chain = true;
               }
               WindowEvent::Moved(_) => {}
               WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
               WindowEvent::Destroyed => {}
               WindowEvent::DroppedFile(_) => {}
               WindowEvent::HoveredFile(_) => {}
               WindowEvent::HoveredFileCancelled => {}
               WindowEvent::ReceivedCharacter(_) => {}
               WindowEvent::Focused(_) => {}
               WindowEvent::KeyboardInput { .. } => {}
               WindowEvent::CursorMoved { .. } => {}
               WindowEvent::CursorEntered { .. } => {}
               WindowEvent::CursorLeft { .. } => {}
               WindowEvent::MouseWheel { .. } => {}
               WindowEvent::MouseInput { .. } => {}
               WindowEvent::TouchpadPressure { .. } => {}
               WindowEvent::AxisMotion { .. } => {}
               WindowEvent::Touch(_) => {}
               WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                   surface_extent = Extent2D{
                       width: new_inner_size.width,
                       height: new_inner_size.height
                   };
                   should_configure_swap_chain = true;
               }
               WindowEvent::ThemeChanged(_) => {}
           }
           Event::DeviceEvent { .. } => {}
           Event::UserEvent(_) => {}
           Event::Suspended => {}
           Event::Resumed => {}
           Event::MainEventsCleared => window.request_redraw(),
           Event::RedrawRequested(_) => {
               //redraw code here
           }
           Event::RedrawEventsCleared => {}
           Event::LoopDestroyed => {}
       }
    });
}
