use easygpu::{euclid::Size2D, prelude::*};
use easygpu_lyon::LyonPipeline;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub trait Sandbox: Sized + 'static {
    fn create(renderer: &Renderer) -> Self;
    fn pipeline(&self) -> &'_ LyonPipeline;
    fn render<'a, 'b>(&'a self, pass: &'b mut easygpu::wgpu::RenderPass<'a>);

    fn run() -> Result<(), easygpu::error::Error> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let size = window.inner_size();

        // Setup renderer
        let instance = easygpu::wgpu::Instance::new(easygpu::wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let mut renderer = futures::executor::block_on(Renderer::new(surface, &instance))?;
        let sandbox = Self::create(&renderer);

        let mut textures = renderer.swap_chain(
            Size2D::new(size.width, size.height).cast::<u32>(),
            PresentMode::default(),
        );

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    textures = renderer.swap_chain(
                        Size2D::new(size.width, size.height).cast::<u32>(),
                        PresentMode::default(),
                    );
                    *control_flow = ControlFlow::Wait;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if let Ok(output) = textures.next_texture() {
                    let mut frame = renderer.frame();

                    renderer.update_pipeline(
                        sandbox.pipeline(),
                        ScreenTransformation::ortho(
                            0.,
                            output.size.width as f32,
                            output.size.height as f32,
                            0.,
                            -1.,
                            1.,
                        ),
                        &mut frame,
                    );

                    {
                        let mut pass = frame.pass(PassOp::Clear(Rgba::TRANSPARENT), &output);

                        sandbox.render(&mut pass);
                    }
                    renderer.present(frame);
                }
            }
            _ => {
                *control_flow = ControlFlow::Wait;
            }
        });
    }
}
