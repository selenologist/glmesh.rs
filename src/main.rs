#[macro_use]
extern crate glium;
extern crate image;
extern crate glm;
extern crate assimp;
extern crate num_traits;

mod mesh;
use mesh::Mesh;

mod shader;
use shader::Shader;

fn unmat(mat: &glm::Matrix4<f32>) -> [[f32; 4]; 4]{
    [*mat.c0.as_array(),
     *mat.c1.as_array(),
     *mat.c2.as_array(),
     *mat.c3.as_array()]
}

fn main() {
    use glium::{DisplayBuild,Surface};
    use glm::ext::*;
    use glm::*;
    use num_traits::identities::One;

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .expect("Failed to create window!");

    let mesh   = Mesh::load  (&display, "suzanne.obj").expect("Failed to load mesh");
    let shader = Shader::load(&display, "Lighting.vert", "Lighting.frag").expect("Failed to load shader");

    loop{
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let (width, height) = display.get_framebuffer_dimensions();

        let proj = perspective(45.0f32, width as f32 / height as f32, 0.1f32, 100.0f32);
        let view = look_at(vec3(0.0f32, 5.0f32, 15.0f32), vec3(0.0f32,0.0f32,0.0f32), vec3(0.0f32,1.0f32,0.0f32));
        let model = translate(&Matrix4::<f32>::one(), vec3(0.0f32, 2.5f32, 10.0f32));

        mesh.draw(&mut target, &shader,
                  &uniform!{ proj:  unmat(&proj),
                             view:  unmat(&view),
                             model: unmat(&model) })
            .expect("Failed to draw");

        target.finish()
              .expect("Failed to finish drawing!");
        
        for ev in display.poll_events() {
            match ev{
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
