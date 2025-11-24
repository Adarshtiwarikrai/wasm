use wasm_bindgen::{prelude::*, JsCast};

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use web_sys::HtmlElement;
use web_sys::{
    console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
    WebGlProgram, WebGlRenderingContext, WebGlShader,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Access the document and canvas
    let window = web_sys::window().ok_or("No global window found")?;
    let document = window.document().ok_or("No document found")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlCanvasElement>()?;

    println!("Hello from wasm!");
    let context = canvas
        .get_context("2d")
        .expect("Could not get context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    console_error_panic_hook::set_once();

    console::log_1(&"Hello from Rust WASM!".into());

    {
        console::log_1(&"Hello from mouse event".into());
        let context = context.clone();
        console::log_1(&"Hello from mouse event 1".into());
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            console::log_1(&"Hello from mouse event 2".into());
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            // context.set_stroke_style_str("blue");
            //  context.stroke_rect(event.offset_x() as f64, event.offset_y() as f64, 10 as f64, 30 as f64);
            //  context.begin_path();
            //  context.arc(event.offset_x() as f64, event.offset_y() as f64,15 as f64,0 as f64,360 as f64);
            //  context.stroke();
            context.set_font("20px Georgia");
            context.stroke_text(
                "Hello Rust",
                event.offset_x() as f64,
                event.offset_y() as f64,
            );

            console::log_1(&"Hello from mouse event 3".into());
            console::log_2(&JsValue::from_f64(new_x), &JsValue::from_f64(new_y));
        }) as Box<dyn FnMut(_)>);
        let key_down = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            
            console::log_1(&"Hello from keyboard event 2".into());
            let key=event.key();
            console::log_1(&key.into());
            console::log_1(&"Hello from keyboard event 2".into());

            console::log_1(&"Hello from keyboard event 3".into());
            
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;
        canvas
            .add_event_listener_with_callback("keydown", key_down.as_ref().unchecked_ref())?;

        mouse_down.forget();
    }
    // Get the WebGL rendering context
    // let gl = canvas
    //     .get_context("webgl")?
    //     .ok_or("Failed to get WebGL context")?
    //     .dyn_into::<WebGlRenderingContext>()?;
    // let aa=gl.drawing_buffer_height();
    // println!("jggjljhlkjhlkhklkk{:?}",aa);
    // // Initialize shaders
    // let vertex_shader = compile_shader(
    //     &gl,
    //     WebGlRenderingContext::VERTEX_SHADER,
    //     r#"
    //     attribute vec4 position;
    //     void main() {
    //         gl_Position = position;
    //     }
    //     "#,
    // )?;

    // let fragment_shader = compile_shader(
    //     &gl,
    //     WebGlRenderingContext::FRAGMENT_SHADER,
    //     r#"
    //     precision mediump float;
    //     void main() {
    //       gl_FragColor = vec4(0.0, 1.0, 1.0, 1.0);

    //     }
    //     "#,
    // )?;

    // let program = link_program(&gl, &vertex_shader, &fragment_shader)?;
    // gl.use_program(Some(&program));

    // // Set up the vertices
    // let vertices: [f32; 6] = [
    //     0.0,  0.5,  // Top vertex
    //    -0.5, -0.5,  // Bottom left vertex
    //     0.5, -0.5,  // Bottom right vertex
    // ];

    // let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    // gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // unsafe {
    //     let vertices_array = js_sys::Float32Array::view(&vertices);
    //     gl.buffer_data_with_array_buffer_view(
    //         WebGlRenderingContext::ARRAY_BUFFER,
    //         &vertices_array,
    //         WebGlRenderingContext::DYNAMIC_DRAW,
    //     );
    // }

    // // Link the position attribute in the vertex shader
    // let position = gl.get_attrib_location(&program, "position") as u32;
    // gl.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    // gl.enable_vertex_attrib_array(position);

    // // Clear the canvas and draw the triangle
    // gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    // gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(())
}

// fn compile_shader(
//     gl: &WebGlRenderingContext,
//     shader_type: u32,
//     source: &str,
// ) -> Result<WebGlShader, String> {
//     let shader = gl
//         .create_shader(shader_type)
//         .ok_or("Unable to create shader object")?;
//     gl.shader_source(&shader, source);
//     gl.compile_shader(&shader);

//     if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(shader)
//     } else {
//         Err(gl.get_shader_info_log(&shader).unwrap_or_default())
//     }
//  }

// fn link_program(
//     gl: &WebGlRenderingContext,
//     vertex_shader: &WebGlShader,
//     fragment_shader: &WebGlShader,
// ) -> Result<WebGlProgram, String> {
//     let program = gl
//         .create_program()
//         .ok_or("Unable to create shader program")?;

//     gl.attach_shader(&program, vertex_shader);
//     gl.attach_shader(&program, fragment_shader);
//     gl.link_program(&program);

//     if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(program)
//     } else {
//         Err(gl.get_program_info_log(&program).unwrap_or_default())
//     }
// }

// cargo build --target wasm32-unknown-unknown
// wasm-bindgen target/wasm32-unknown-unknown/debug/rust_webgl_example.wasm \
//   --out-dir pkg --target web
// asdfasdfasdfassdfasdfadsfa
//python -m http.server 8000
