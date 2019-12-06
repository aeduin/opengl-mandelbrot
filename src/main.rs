#[macro_use]
extern crate glium;

mod math;

use glium::{glutin, Surface};
use glutin::*;
use math::Vertex;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    //open window and initialise
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_min_dimensions(glutin::dpi::LogicalSize{
        width: 500.0,
        height: 400.0,
    }).with_title("OpenGL Mandelbrot");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();


    let (stdin_sender, stdin_receiver) = channel();

    thread::spawn(move || {
        let reader = io::stdin();

        for line in reader.lock().lines() {
            stdin_sender.send(line.unwrap()).unwrap();
        }
    });
    
    
    //initialize vertex buffers
    let (index_buffer, vertex_buffer) = {
        let vertices = [
            Vertex {
                position: math::Vector3{x: -1.0, y: -1.0, z: 0.0},
                texture_coordinate: math::Vector2{x: -1.0, y: -1.0},
            },
            Vertex {
                position: math::Vector3{x: 1.0, y: -1.0, z: 0.0},
                texture_coordinate: math::Vector2{x: 1.0, y: -1.0},
            },
            Vertex {
                position: math::Vector3{x: -1.0, y: 1.0, z: 0.0},
                texture_coordinate: math::Vector2{x: -1.0, y: 1.0},
            },
            Vertex {
                position: math::Vector3{x: 1.0, y: 1.0, z: 0.0},
                texture_coordinate: math::Vector2{x: 1.0, y: 1.0},
            },
        ];

        let indices: [u16; 6] = [
            0, 1, 2,
            1, 2, 3
        ];

        (glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(), glium::vertex::VertexBuffer::new(&display, &vertices).unwrap())
    };
    
    let program = glium::Program::from_source(&display, include_str!("triangle.vert"), include_str!("triangle.frag"), None).unwrap();

    //initialize variables for main loop
    let mut scale: f64 = 2.0;
    let mut center: [f64; 2] = [0.0, 0.0];
    let mut mouse_position: (f64, f64) = (0.0, 0.0);
    let mut window_size: (f64, f64) = (500.0, 400.0);
    let mut need_draw_update = true;
    let mut max_mandel_number: f32 = 2000.0;
    let mut zooming = false;

    let mut open = true;
 
    let mut last_spacebar_update = std::time::SystemTime::now();
    let mut last_zoom_update = std::time::SystemTime::now();
    let mut draw_start;

    //start main loop
    while open {
        if zooming {
            let elapsed = last_zoom_update.elapsed().unwrap();
            let mut elapsed_milli_seconds = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;

            while elapsed_milli_seconds > 0 {
                elapsed_milli_seconds -= 1;
                scale *= 0.9998;
            }
            need_draw_update = true;
            last_zoom_update = std::time::SystemTime::now();
        }
        //draw
        if need_draw_update {
            draw_start = std::time::SystemTime::now();

            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let draw_parameters = &glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            target.draw(&vertex_buffer, &index_buffer, &program, &uniform!{
                center: center,
                scale: scale,
                max_mandel_number: max_mandel_number,
                x_scale: if aspect_ratio > 1.0 {aspect_ratio} else {1.0},
                y_scale: if aspect_ratio < 1.0 {1.0 / aspect_ratio} else {1.0},
            }, &draw_parameters).unwrap();
            
            target.finish().unwrap();

            //print draw time
            let elapsed = draw_start.elapsed().unwrap();
            // println!("drawing took {}ms", elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64);

            need_draw_update = false;
        }
        
        // Read stdin
        match stdin_receiver.try_recv() {
            Err(_error) => (),
            Ok(input) => {
                for command in subdivide_commands(&input) {
                    match command {
                        Command::Invalid => println!("error, invalid comand"),
                        Command::Update{variable_name, value, action} => {
                            if *variable_name == *"s" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => action.apply(&mut scale, parsed_value),
                                }
                            }
                            else if variable_name == "x" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => action.apply(&mut center[0], parsed_value),
                                }
                            }
                            else if variable_name == "y" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => action.apply(&mut center[1], parsed_value),
                                }
                            }
                            else if variable_name == "i" {
                                match value.parse::<f32>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => action.apply(&mut max_mandel_number, parsed_value),
                                }
                            }
                            else {
                                println!("unknown variable name: {}", variable_name);
                            }
                        },
                        Command::Set{variable_name, value} => {
                            if variable_name == "s" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => scale = parsed_value,
                                }
                            }
                            else if variable_name == "x" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => center[0] = parsed_value,
                                }
                            }
                            else if variable_name == "y" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => center[1] = parsed_value,
                                }
                            }
                            else if variable_name == "i" {
                                match value.parse::<f32>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => max_mandel_number = parsed_value,
                                }
                            }
                            else {
                                println!("unknown variable name: {}", variable_name);
                            }
                        },
                        Command::ToggleZoom => zooming = !zooming,
                        Command::Export => 
                            println!("x={},y={},s={},i={}", center[0], center[1], scale, max_mandel_number),
                        
                    }
                }

                // if input.len() > 2 {
                //     // let input = &input[..];
    
                //     if input[0..2] == *"i=" {
                //         let head = &input[..2];
                //         let tail = &input[2..];
                //         match tail.parse::<f32>() {
                //             Err(_error) => println!("invalid number: {}", input),
                //             Ok(new_max) => {
                //                 need_draw_update = true;
                //                 max_mandel_number = new_max;
                //             },
                //         };
                //     }
                //     else if input[0..3] == *"s*=" {
                //         let head = &input[..3];
                //         let tail = &input[3..];
                //         match tail.parse::<f64>() {
                //             Err(_error) => println!("invalid number: {}", input),
                //             Ok(multiplier) => {
                //                 need_draw_update = true;
                //                 scale *= multiplier;
                //             },
                //         };
                //     }
                //     else {
                //         println!("invalid input: {}", input);
                //     }
                // }
                // else {
                //     println!("input to short: {}", input);
                // }

                need_draw_update = true;
            }
        }

        //poll events
        {
            events_loop.poll_events(|ev| {
                match ev {
                    Event::WindowEvent {event, ..} => match event {
                        WindowEvent::CloseRequested => open = false,
                        WindowEvent::CursorMoved{position, ..} => {
                            mouse_position = (position.x as f64, position.y as f64);
                        },
                        WindowEvent::MouseInput{state, button, ..} => {
                            if state == ElementState::Pressed && button == MouseButton::Left {
                                // println!("centering!");
                                center = pixel_to_mandel_coords((center[0], center[1]), window_size, mouse_position, scale);
                                need_draw_update = true;
                            }
                        },
                        WindowEvent::Resized(size) => {
                            window_size = (size.width as f64, size.height as f64);
                            need_draw_update = true;
                        },
                        _ => (),
                    },
                    glutin::Event::DeviceEvent {event, ..} => match event {
                        glutin::DeviceEvent::MouseWheel {delta} => match delta {
                            glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                //println!("scrollY value: {}", y);
                                scale *= 1.0 - (y as f64) / 50.0;
                                need_draw_update = true;
                            },
                            _ => (),
                        },
                        glutin::DeviceEvent::Key(key) => {
                            match key.virtual_keycode {
                                Some(key_code) => {
                                    match key_code {
                                        glutin::VirtualKeyCode::Space => {
                                            if false {
                                                let elapsed_time = last_spacebar_update.elapsed().unwrap();
                                                if elapsed_time.subsec_millis() > 300 || elapsed_time.as_secs() > 0 {
                                                    println!("spacebar pressed");
    
                                                    zooming = !zooming;
    
                                                    if zooming {
                                                        last_zoom_update = std::time::SystemTime::now();
                                                    }
    
                                                    last_spacebar_update = std::time::SystemTime::now();
                                                }
                                            }
                                        }
                                        _ => ()
                                    }
                                },
                                None => (), //println!("no virtual keycode for pressed key"),
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            })
        }
        std::thread::sleep(std::time::Duration::from_micros(1));
    }
}

enum Action {
    Multiply,
    Divide,
    Add,
    Subtract,
}

trait Apply<T> {
    fn apply(&self, target: &mut T, value: T) ;
}

impl<T> Apply<T> for Action where T: std::ops::Add<T, Output=T> + std::ops::Div<T, Output=T> + std::ops::Sub<T, Output=T> + std::ops::Mul<T, Output=T> + Copy{
    fn apply(&self, target: &mut T, value: T) {
        match self {
            Action::Multiply => *target = *target * value,
            Action::Divide => *target = *target / value,
            Action::Add => *target = *target + value,
            Action::Subtract => *target = *target - value,
        }
    }
}

enum Command {
    Update{variable_name: String, value: String, action: Action},
    Set{variable_name: String, value: String},
    Export,
    ToggleZoom,
    Invalid,
}

impl<'a> From<&'a String> for Command {
    fn from(input: &'a String) -> Command {
        for i in 0..input.len() {
            if input[i..i+1] == *"=" {
                // type is Command::Update or Command::Set
                let action =
                    if input[i-1..i] == *"*" {
                        Action::Multiply
                    }
                    else if input[i-1..i] == *"/" {
                        Action::Divide
                    }
                    else if input[i-1..i] == *"+" {
                        Action::Add
                    }
                    else if input[i-1..i] == *"-" {
                        Action::Subtract
                    }
                    else {
                        return Command::Set{variable_name: String::from(&input[0..i]), value: String::from(&input[i+1..])};
                    };
    
                return Command::Update{variable_name: String::from(&input[0..i-1]), value: String::from(&input[i+1..]), action};
            }
        }
        // no '='-sign found
        if input == "export" {
            Command::Export
        }
        else if input == "zoom" {
            Command::ToggleZoom
        }
        else{
            Command::Invalid
        }
    }
}

fn subdivide_commands<'a>(comma_seperated: &'a String) -> Vec<Command> {
    let mut substrings = Vec::<Command>::new();

    let mut begin = 0;

    for i in 0..comma_seperated.len() {
        if comma_seperated[i..i+1] == *"," {
            let string = filter_spaces(&comma_seperated[begin..i]);

            substrings.push(
                Command::from(&string)
            );

            begin = i + 1;
        }
    }

    let i = comma_seperated.len();
    let string = filter_spaces(&comma_seperated[begin..i]);

    substrings.push(
        Command::from(&string)
    );

    substrings
}

fn filter_spaces<'a>(in_str: &str) -> String {
    let mut result = String::new();

    for c in in_str.chars() {
        if c != ' ' {
            result.push(c);
        }
    }

    result
}

fn pixel_to_mandel_coords((center_x, center_y): (f64, f64), (screen_size_width, screen_size_height): (f64, f64), (pixel_x, pixel_y): (f64, f64), scale: f64) -> [f64; 2] {
    let x_scale: f64 = if screen_size_width < screen_size_height {screen_size_height / screen_size_width} else {1.0};
    let y_scale: f64 = if screen_size_width > screen_size_height {screen_size_width / screen_size_height} else {1.0};

    [
        center_x + (pixel_x / screen_size_width * 2.0 - 1.0) * scale / x_scale,
        center_y - (pixel_y / screen_size_height * 2.0 - 1.0) * scale / y_scale,
    ]
}
