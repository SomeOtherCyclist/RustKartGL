use glium::*;
use winit::{event::{ElementState, WindowEvent as WindowEvent}, platform::modifier_supplement::KeyEventExtModifierSupplement};
use gilrs::{Gilrs, Event};

extern crate glium;

mod load_shaders;
mod controller;
mod camera;
mod car;

mod teapot;

#[allow(unused_mut)]
fn main() {
    let controls_thread = std::thread::spawn(move || {
        let mut car = car::Car::default().player();
        let mut gilrs = Gilrs::new().unwrap();
        let mut controller = controller::ControllerEvent::default();

        let a = gilrs;
        //gilrs;
        //while let Some(Event {event, ..}) = gilrs.next_event() {
            //println!("Looping!")
        //}
        a
    });

    let b = controls_thread.join().unwrap();

    //gilrs.next_event();

    let mut camera = camera::new();

    let mut now = std::time::Instant::now();

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop builder had a problem");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Controller input hurt my head")
        .build(&event_loop);

    let _ = winit::window::Window::set_cursor_grab(&window, winit::window::CursorGrabMode::Locked);
    winit::window::Window::set_cursor_visible(&window, false);

    let mut time: f32 = 0.0;
    let mut dt = 1.0;

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let vertex_shader_src = load_shaders::load_shader("basic_vertex.vs");
    let fragment_shader_src = load_shaders::load_shader("basic_fragment.fs");
    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit()
                },
                WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into())
                },
                WindowEvent::RedrawRequested => {
                    //Initilise target
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                    println!("{}", car.drivetrain.gearbox.gear);

                    //Last frame time calculation
                    dt = (std::time::Instant::now() - now).as_secs_f32();
                    now = std::time::Instant::now();
                    time += dt;

                    let model = [
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 2.0 + time.sin(), 1.0f32]
                    ];
                    let light = [time.sin(), 0.4, time.cos()];
                    let perspective = camera::perspective(&target);
                    let view = camera::view(&camera);

                    let uniforms = uniform! {
                        model: model,
                        perspective: perspective,
                        u_light: light,
                        view: view
                    };

                    target.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();
                    target.finish().unwrap();

                    //dog poop code to delay the next frame so that it runs at 60fps
                    let duration = std::time::Duration::new(0, 16666666);
                    std::thread::sleep( duration);
                },
                WindowEvent::KeyboardInput {event, .. } => {
                    //match event.physical_key {}                                                          VERY IMPORTANT CHANGE TO MAKE SO THAT MY CODE IS NOT SUCH HOT GARBAGE
                },
                _ => ()
            }
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => ()
        };
    });
}
