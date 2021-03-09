use std::sync::Arc;

use crate::shape::{Shape, Vertex};
use easygpu::{color::Rgba, euclid::Vector3D, renderer::Renderer, transform::ScreenSpace};
use lyon_tessellation::{
    math::Point, path::Path, FillOptions, FillTessellator, GeometryBuilderError, StrokeOptions,
    StrokeTessellator, TessellationError, VertexId,
};

mod lyon_builders;

#[derive(Default, Debug)]
/// Builds a shape using lyon for tesselation
pub struct ShapeBuilder {
    zdepth: f32,
    vertices: Vec<Vertex>,
    indicies: Vec<u16>,

    /// This RGBA color is used when tesselating a path with no color data (Attributes in lyon terminology)
    pub default_color: [f32; 4],
}

impl ShapeBuilder {
    /// Create a new ShapeBuilder with a given ZDepth
    ///
    /// # Arguments
    ///
    /// * `zdepth` - The z depth for shapes in this builder to have
    pub fn new(zdepth: f32, default_color: [f32; 4]) -> Self {
        Self {
            zdepth,
            default_color,
            ..Default::default()
        }
    }

    /// Prepare and load this builder into the renderer.
    ///
    /// This does not consume the builder, because wgpu copies the buffer rather than taking ownerhip.
    pub fn prepare(&self, renderer: &Renderer) -> Shape {
        let verticies = renderer.device.create_buffer(&self.vertices);
        let indicies = renderer.device.create_index(&self.indicies);

        Shape {
            index_count: self.indicies.len() as u32,
            vertices: Arc::new(verticies),
            indices: Arc::new(indicies),
        }
    }

    /// Fill an arbitrary path from `lyon::path`
    pub fn fill(&mut self, path: &Path, options: &FillOptions) -> Result<(), TessellationError> {
        let mut tesselator = FillTessellator::new();
        let _ = tesselator.tessellate_with_ids(path.id_iter(), path, Some(path), options, self)?;
        Ok(())
    }

    /// Stroke an arbitrary path from `lyon::path`
    pub fn stroke(
        &mut self,
        path: &Path,
        options: &StrokeOptions,
    ) -> Result<(), TessellationError> {
        let mut tesselator = StrokeTessellator::new();
        let _ = tesselator.tessellate_with_ids(path.id_iter(), path, Some(path), options, self)?;
        Ok(())
    }

    fn new_vertex(&mut self, point: Point, attributes: &[f32]) -> Vertex {
        let attributes = if attributes.is_empty() {
            &self.default_color
        } else {
            attributes
        };

        assert!(attributes.len() == 4, "Attributes should be RGBA");

        Vertex {
            color: Rgba {
                r: attributes[0],
                g: attributes[1],
                b: attributes[2],
                a: attributes[3],
            }
            .into(),
            position: Vector3D::<f32, ScreenSpace>::new(point.x, point.y, self.zdepth).to_array(),
        }
    }

    fn add_vertex(
        &mut self,
        point: Point,
        attributes: &[f32],
    ) -> Result<VertexId, GeometryBuilderError> {
        let vertex = self.new_vertex(point, attributes);
        let new_id = VertexId(self.vertices.len() as u32);
        self.vertices.push(vertex);
        if self.vertices.len() > u16::MAX as usize {
            return Err(GeometryBuilderError::TooManyVertices);
        }

        Ok(new_id)
    }
}
