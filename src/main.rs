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
use std::fs;

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

        loop {
            for line in reader.lock().lines() {
                stdin_sender.send(line.unwrap()).unwrap();
            }
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
    
    // let program = glium::Program::from_source(&display, include_str!("triangle.vert"), include_str!("triangle.frag"), None).unwrap();
    let mut program = match program_from_shaders(&display) {
        Some(value) => value,
        None => panic!("can't construct shader"),
    };

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

    let mut zoom_scale = 0.0002;
    let mut color_function_id: i32 = 0;
    let continuous_color_functions = [4, 5, 6];

    let program_start = std::time::SystemTime::now();

    //start main loop
    while open {
        if zooming {
            let elapsed = last_zoom_update.elapsed().unwrap();
            let mut elapsed_milli_seconds = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;

            while elapsed_milli_seconds > 0 {
                elapsed_milli_seconds -= 1;
                scale *= 1.0 - zoom_scale;
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

            let elapsed_since_start = program_start.elapsed().unwrap();
            let seconds_since_start = elapsed_since_start.as_secs() as f32 + elapsed_since_start.subsec_millis() as f32 / 1000.0;
            // let seconds_since_start = seconds_since_start as f32 / 1000.0;
            // println!("seconds since start = {}", seconds_since_start);
            
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
                color_function_id: color_function_id,
                time: seconds_since_start,
            }, &draw_parameters).unwrap();
            
            target.finish().unwrap();

            //print draw time
            // let elapsed = draw_start.elapsed().unwrap();
            // println!("drawing took {}ms", elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64);
            if !continuous_color_functions.contains(&color_function_id) {
                need_draw_update = false;
            }
        }
        
        // Read stdin
        match stdin_receiver.try_recv() {
            Err(_error) => (),
            Ok(input) => {
                for (command_string, command) in subdivide_commands(&input) {
                    match command {
                        Command::Invalid => println!("error, invalid comand: \"{}\"", command_string),
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
                            else if variable_name == "z" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => action.apply(&mut zoom_scale, parsed_value),
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
                            else if variable_name == "z" {
                                match value.parse::<f64>() {
                                    Err(_error) => println!("invalid number format: {}", value),
                                    Ok(parsed_value) => zoom_scale = parsed_value,
                                }
                            }
                            else if variable_name == "c" {
                                match value.parse::<i32>() {
                                    Err(_error) => println!("invalid integer number format: {}", value),
                                    Ok(parsed_value) => {
                                        color_function_id = parsed_value;
                                        if continuous_color_functions.contains(&color_function_id) {
                                            need_draw_update = true;
                                        }
                                    },
                                }
                            }
                            else {
                                println!("unknown variable name: {}", variable_name);
                            }
                        },
                        Command::ToggleZoom => {
                            if zooming {
                                zooming = false;
                            }
                            else {
                                zooming = true;
                                last_zoom_update = std::time::SystemTime::now();
                            }
                        }
                        Command::Export => 
                            println!("x={},y={},s={},i={},c={}", center[0], center[1], scale, max_mandel_number, color_function_id),
                        Command::ReloadShader => {
                            match program_from_shaders(&display) {
                                Some(value) => program = value,
                                None => (),
                            }
                        }
                        
                    }
                }

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

fn program_from_shaders(display: &glium::Display) -> Option<glium::Program> {
    let vertex_file = match fs::read_to_string("src/triangle.vert") {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading vertex shader triangle.vert. Error message: {}", error);
            return None;
        }
    };
    let fragment_file = match fs::read_to_string("src/triangle.frag") {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading fragment shader triangle.frag. Error message: {}", error);
            return None;
        }
    };;

    let program = glium::Program::from_source(display, &vertex_file[..], &fragment_file[..], None).unwrap();

    Some(program)
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
    ReloadShader,
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
        else if input == "reloadshader" {
            Command::ReloadShader
        }
        else{
            Command::Invalid
        }
    }
}

fn subdivide_commands<'a>(comma_seperated: &'a String) -> Vec<(String, Command)> {
    let mut substrings = Vec::new();

    let mut begin = 0;

    for i in 0..comma_seperated.len() + 1 {
        let end_of_line =
            i == comma_seperated.len() ||
            comma_seperated[i..i+1] == *"#";

        if end_of_line || comma_seperated[i..i+1] == *"," {
            let command_string = filter_spaces(&comma_seperated[begin..i]);
            let command = Command::from(&command_string);

            substrings.push(
                (command_string, command)
            );

            begin = i + 1;
        }
        if end_of_line {
            break;
        }
    }

    // let i = comma_seperated.len();
    // let string = filter_spaces(&comma_seperated[begin..i]);

    // substrings.push(
    //     Command::from(&string)
    // );

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
