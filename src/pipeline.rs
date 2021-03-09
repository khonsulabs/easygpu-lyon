use bytemuck::{Pod, Zeroable};
use easygpu::prelude::*;
use std::ops::Deref;

/// A pipeline for rendering shapes.
pub struct LyonPipeline {
    pipeline: PipelineCore,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
/// The uniforms for the shader.
pub struct Uniforms {
    /// The orthographic projection matrix
    pub ortho: [f32; 16],
    /// The transformation matrix
    pub transform: [f32; 16],
}

impl<'a> AbstractPipeline<'a> for LyonPipeline {
    type PrepareContext = ScreenTransformation<f32>;
    type Uniforms = Uniforms;

    fn description() -> PipelineDescription<'a> {
        PipelineDescription {
            vertex_layout: &[VertexFormat::Float3, VertexFormat::UByte4],
            pipeline_layout: &[Set(&[Binding {
                binding: BindingType::UniformBuffer,
                stage: ShaderStage::VERTEX,
            }])],
            vertex_shader: include_bytes!("shaders/shape.vert.spv"),
            fragment_shader: include_bytes!("shaders/shape.frag.spv"),
        }
    }

    fn setup(pipeline: Pipeline, dev: &Device) -> Self {
        let transform = ScreenTransformation::identity().to_array();
        let ortho = ScreenTransformation::identity().to_array();
        let uniforms = dev.create_uniform_buffer(&[self::Uniforms { ortho, transform }]);
        let bindings = dev.create_binding_group(&pipeline.layout.sets[0], &[&uniforms]);

        Self {
            pipeline: PipelineCore {
                pipeline,
                uniforms,
                bindings,
            },
        }
    }

    fn prepare(
        &'a self,
        ortho: Self::PrepareContext,
    ) -> Option<(&'a UniformBuffer, Vec<Self::Uniforms>)> {
        let ortho = ortho.to_array();
        let transform = ScreenTransformation::identity().to_array();
        Some((
            &self.pipeline.uniforms,
            vec![self::Uniforms { transform, ortho }],
        ))
    }
}

impl Deref for LyonPipeline {
    type Target = PipelineCore;
    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}
