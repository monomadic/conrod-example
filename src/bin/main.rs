#[macro_use] extern crate conrod;
extern crate winit;

use conrod::backend::glium::glium::glutin::{WindowBuilder};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

mod app;
pub use app::App;

fn main() {
    // using winit:
    // let wb = winit::WindowBuilder::new();

    // let display = WindowBuilder::from_winit_builder(wb)
    //     .with_vsync()
    //     .with_dimensions(640, 480)
    //     .with_title("Conrod with glium!")
    //     .with_multisampling(8)
    //     .build_glium()
    //     .unwrap();

    // using glium:
    let display = WindowBuilder::new()
        .with_vsync()
        .with_dimensions(640, 480)
        .with_title("Conrod with glium!")
        .with_multisampling(8)
        .build_glium()
        .unwrap();
    
    let mut app = App::new(display).unwrap();

    loop {
        app.draw();
    }
}
