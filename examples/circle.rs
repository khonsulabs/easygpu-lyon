use easygpu::prelude::*;
use easygpu_lyon::{LyonPipeline, Shape, ShapeBuilder, Srgb, VertexShaderSource};
use lyon_tessellation::{basic_shapes::fill_circle, math::Point, FillOptions};

mod sandbox;
use sandbox::Sandbox;

fn main() -> Result<(), easygpu::error::Error> {
    CircleExample::run()
}

struct CircleExample {
    pipeline: LyonPipeline<Srgb>,
    shape: Shape,
}

impl Sandbox for CircleExample {
    fn create(renderer: &Renderer) -> Self {
        let pipeline = renderer.pipeline(Blending::default(), Srgb::sampler_format());

        let mut builder = ShapeBuilder::default();
        builder.default_color = [1., 0., 0., 1.];

        fill_circle(
            Point::new(50., 50.),
            25.,
            &FillOptions::default(),
            &mut builder,
        )
        .expect("Error tesselating circle");
        let shape = builder.prepare(&renderer);

        Self { pipeline, shape }
    }

    fn pipeline(&self) -> &'_ LyonPipeline<Srgb> {
        &self.pipeline
    }

    fn render<'a, 'b>(&'a self, pass: &'b mut easygpu::wgpu::RenderPass<'a>) {
        pass.set_easy_pipeline(&self.pipeline);
        self.shape.draw(pass);
    }
}
