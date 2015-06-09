/**
 *  Developing area
 */

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate rand;

use gfx::traits::{Stream, ToIndexSlice, ToSlice, FactoryExt};

// define OpenGL vertexes
gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 2],
    a_Color@ color: [f32; 3],
});

pub fn main() {

    // toulpe window
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(glutin::Window::new().unwrap());
    // title the window
    stream.out.window.set_title("Dev");

    'main: loop {

        // define vertex
        let vertex_data = [
            Vertex { pos: [0.0f32,0.0f32], color: [1.0f32,0.0f32,1.0f32] },
            Vertex { pos: [0.1f32,0.0f32], color: [1.0f32,0.0f32,1.0f32] },
            Vertex { pos: [0.0f32,0.2f32], color: [1.0f32,0.0f32,1.0f32] },
            Vertex { pos: [0.3f32,0.3f32], color: [1.0f32,0.0f32,0.0f32] },
            Vertex { pos: [0.3f32,0.4f32], color: [1.0f32,0.0f32,0.0f32] },
        ];

        // create mesh out of vertx
        let mesh = factory.create_mesh(&vertex_data);
        //println!(" Mesh {:?}\n",&vertex_data);

        // connect points
        let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip );
        //println!(" Slice {:?}\n",&slice);

        // define the OpenGL program
        let program = {
            let vs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslv")),
                glsl_150: Some(include_bytes!("triangle_150.glslv")),
                .. gfx::ShaderSource::empty()
            };
            let fs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslf")),
                glsl_150: Some(include_bytes!("triangle_150.glslf")),
                .. gfx::ShaderSource::empty()
            };
            factory.link_program_source(vs, fs).unwrap()
        };
        //println!(" OpenGL program {:?}\n",&program);

        // Create new draw state
        let state = gfx::DrawState::new();

        for event in stream.out.window.poll_events() {
            match event {
                // quit when Esc is pressed.
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                // debug on spacekey
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Space)) => println!(" Vertex R: {:?} \n Vertex G: {:?} \n Vertex B: {:?}\n Vertex Y: {:?}\n {:?}\n{:?}\n",&vertex_data[0].pos,&vertex_data[1].pos,&vertex_data[2].pos,&vertex_data[2].pos,&mesh,&slice),
                // else breake
                glutin::Event::Closed => break 'main,
                // always execute
                _ => {},
            }
        }

        // clear the content and redraw the BG
        stream.clear(gfx::ClearData {
            color: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
            depth: 1.0f32,
            stencil: 0,
        });
        // draw it over the BG
        stream.draw(&gfx::batch::bind(&state, &mesh, slice.clone(), &program, &None)).unwrap();

        //println!(" OpenGL program {:?}\n",&gfx::batch::bind(&state, &mesh, slice.clone(), &program, &None));

        // ???
        stream.present(&mut device);
    }
}
