use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, HtmlCanvasElement, WebGlShader, WebGlProgram};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Access the document and canvas
    let window = web_sys::window().ok_or("No global window found")?;
    let document = window.document().ok_or("No document found")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Get the WebGL rendering context
    let gl = canvas
        .get_context("webgl")?
        .ok_or("Failed to get WebGL context")?
        .dyn_into::<WebGlRenderingContext>()?;

    // Initialize shaders
    let vertex_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
        "#,
    )?;

    let fragment_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        void main() {
          gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);

        }
        "#,
    )?;

    let program = link_program(&gl, &vertex_shader, &fragment_shader)?;
    gl.use_program(Some(&program));

    // Set up the vertices
    let vertices: [f32; 6] = [
        0.0,  0.5,  // Top vertex
       -0.5, -0.5,  // Bottom left vertex
        0.5, -0.5,  // Bottom right vertex
    ];

    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertices_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    // Link the position attribute in the vertex shader
    let position = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    // Clear the canvas and draw the triangle
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(())
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_default())
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or("Unable to create shader program")?;
    
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}
