use glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal:   [f32; 3],
    tangent:  [f32; 3],
    texture:  [f32; 2],
}
implement_vertex!(Vertex, position, normal, tangent, texture);

pub struct Mesh{
    vbo: glium::vertex::VertexBuffer<Vertex>,
    ibo: glium::index::IndexBuffer<u32>
}

use std::fmt::Debug;
use std::fmt;
impl Debug for Mesh{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mesh")
    }
}

impl Mesh{
    pub fn load<F: glium::backend::Facade>(display: &F, path: &str) -> Result<Mesh, String>{
        use assimp as ai;
        use assimp::Process::*;
        use glium::index::PrimitiveType::*;

        let mut importer = ai::Importer::new();
        importer.add_processing_steps(&[CalcTangentSpace,
                                        GenNormals,
                                        GenUVCoords,
                                        Triangulate,
                                        OptimizeMeshes,
                                        ImproveCacheLocality,
                                        ValidateDataStructure]);
        let scene = importer.import_from_file(path)
                            .expect("Failed to load mesh");

        if scene.num_meshes < 1 {
            return Err(String::from("File contained no meshes"));
        }

        let ref mesh = scene.get_meshes()[0];

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices:  Vec<u32>    = Vec::new();
        
        for vertex in 0_usize..mesh.num_vertices as usize{
            let ref v = mesh.get_vertices()[vertex];
            let ref n = mesh.get_normals ()[vertex];
            let ref t = mesh.get_tangents()[vertex];
            let ref u = mesh.get_texture_coords()[0][vertex];
            vertices.push(Vertex{
                position: [v.x, v.y, v.z],
                normal:   [n.x, n.y, n.z],
                tangent:  [t.x, t.y, t.z],
                texture:  [u.x, u.y]});
        }

        for face in mesh.get_faces(){
            if face.num_indices != 3{
                return Err(String::from("Non-triangular face in mesh"))
            }
            indices.extend_from_slice(face.get_indices());
        }

        Ok(Mesh{
            vbo: glium::vertex::VertexBuffer::new(display, vertices.as_slice()).expect("Failed to generate VBO"),
            ibo: glium::index::IndexBuffer::new(display, TrianglesList, indices.as_slice()).expect("Failed to generate IBO")
        })
    }
    pub fn draw<S, U>(&self, target: &mut S, shader: &glium::program::Program,
                      uniforms: &U) -> Result<(), glium::DrawError>
        where S: glium::Surface,
              U: glium::uniforms::Uniforms
    {
        use glium::Surface;
        target.draw(&self.vbo, &self.ibo, shader, uniforms, &Default::default())
    }
}
