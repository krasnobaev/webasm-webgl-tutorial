extern crate wasm_bindgen;
extern crate web_sys;
extern crate rustfft;
extern crate serde_derive;
extern crate js_sys;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
  // console,
  WebGlBuffer,
  WebGlProgram,
  WebGlRenderingContext,
  WebGlShader,
  WebGlUniformLocation,
};

use js_sys::WebAssembly;

extern crate nalgebra_glm as glm;
use glm::Mat4;

static VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
static FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");

pub fn draw (
  context: &WebGlRenderingContext,
  width: f32,
  height: f32,
) -> Result<(), JsValue> {
  let vert_shader = compile_shader(
      &context,
      WebGlRenderingContext::VERTEX_SHADER,
      VERTEX_SHADER,
  )?;
  let frag_shader = compile_shader(
      &context,
      WebGlRenderingContext::FRAGMENT_SHADER,
      FRAGMENT_SHADER,
  )?;

  // Initialize a shader program; this is where all the lighting
  // for the vertices and so forth is established.
  let program = link_program(&context, [vert_shader, frag_shader].iter())?;

  // Collect all the info needed to use the shader program.
  // Look up which attribute our shader program is using
  // for aVertexPosition and look up uniform locations.
  let vertex_position: u32 = context.get_attrib_location(&program, "aVertexPosition") as u32;
  let vertex_color: u32 = context.get_attrib_location(&program, "aVertexColor") as u32;
  let projection_matrix = context.get_uniform_location(&program, "uProjectionMatrix");
  let model_view_matrix = context.get_uniform_location(&program, "uModelViewMatrix");

  // Here's where we call the routine that builds all the
  // objects we'll be drawing.
  let buffers = init_buffers(context)?;

  // Draw the scene
  draw_scene(context,
      &program, vertex_position, vertex_color,
      projection_matrix.as_ref(), model_view_matrix.as_ref(),
      buffers, width, height
  )?;

  Ok(())
}

/// Initialize the buffers we'll need. For this demo, we just
/// have one object -- a simple two-dimensional square.
pub fn init_buffers(
  context: &WebGlRenderingContext
) -> Result<[WebGlBuffer; 2], JsValue> {

  // Now create an array of positions for the square
  let positions: [f32; 8] = [
     1.0,  1.0,
    -1.0,  1.0,
     1.0, -1.0,
    -1.0, -1.0,
  ];

  // Create a buffer for the square's positions.
  let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()?
      .buffer();
  let positions_location = positions.as_ptr() as u32 / 4;
  let vert_array = js_sys::Float32Array::new(&memory_buffer)
      .subarray(positions_location, positions_location + positions.len() as u32);
  let buffer1 = context.create_buffer().ok_or("failed to create buffer")?;

  // Select the positionBuffer as the one to apply buffer
  // operations to from here out.
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer1));

  // Now pass the list of positions into WebGL to build the
  // shape. We do this by creating a Float32Array from the
  // JavaScript array, then use it to fill the current buffer.
  context.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGlRenderingContext::STATIC_DRAW,
  );

  // Now create an array of positions for the square
  let colors: [f32; 16] = [
    1.0,  1.0,  1.0,  1.0,    // white
    1.0,  0.0,  0.0,  1.0,    // red
    0.0,  1.0,  0.0,  1.0,    // green
    0.0,  0.0,  1.0,  1.0,    // blue
  ];

  // Create a buffer for the square's positions.
  let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()?
      .buffer();
  let colors_location = colors.as_ptr() as u32 / 4;
  let colors_array = js_sys::Float32Array::new(&memory_buffer)
      .subarray(colors_location, colors_location + colors.len() as u32);
  let buffer2 = context.create_buffer().ok_or("failed to create buffer")?;

  // Select the positionBuffer as the one to apply buffer
  // operations to from here out.
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer2));
  context.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &colors_array,
      WebGlRenderingContext::STATIC_DRAW,
  );

  Ok([buffer1, buffer2])
}

pub fn draw_scene(
  context: &WebGlRenderingContext,
  program: &WebGlProgram,
  vertex_position: u32,
  vertex_color: u32,
  unf_projection_matrix: Option<&WebGlUniformLocation>,
  unf_model_view_matrix: Option<&WebGlUniformLocation>,
  buffers: [WebGlBuffer; 2],
  width: f32,
  height: f32,
) -> Result<(), JsValue> {
  // Clear to black, fully opaque
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  // Clear everything
  context.clear_depth(1.0);
  // Enable depth testing
  context.enable(WebGlRenderingContext::DEPTH_TEST);
  // Near things obscure far things
  context.depth_func(WebGlRenderingContext::LEQUAL);

  // Clear the canvas before we start drawing on it.
  context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

  // Create a perspective matrix, a special matrix that is
  // used to simulate the distortion of perspective in a camera.
  // Our field of view is 45 degrees, with a width/height
  // ratio that matches the display size of the canvas
  // and we only want to see objects between 0.1 units
  // and 100 units away from the camera.
  let field_of_view = 45.0 * std::f32::consts::PI / 180.0;   // in radians
  let aspect = width / height;
  let z_near = 0.1;
  let z_far = 100.0;
  let projection_matrix = glm::perspective(field_of_view, aspect, z_near, z_far);

  // Set the drawing position to the "identity" point, which is
  // the center of the scene.
  let mut model_view_matrix = Mat4::identity();

  // Now move the drawing position a bit to where we want to
  // start drawing the square.
  let translation = glm::vec3(-0.0, 0.0, -6.0);
  model_view_matrix = glm::translate(&model_view_matrix, &translation);

  // Tell WebGL how to pull out the positions from the position
  // buffer into the vertexPosition attribute.
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffers[0]));
  context.vertex_attrib_pointer_with_i32(
      vertex_position,
      2,
      WebGlRenderingContext::FLOAT,
      false,
      0,
      0
  );
  context.enable_vertex_attrib_array(vertex_position);

  // Tell WebGL how to pull out the colors from the color buffer
  // into the vertexColor attribute.
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffers[1]));
  context.vertex_attrib_pointer_with_i32(
      vertex_color,
      4,
      WebGlRenderingContext::FLOAT,
      false,
      0,
      0
  );
  context.enable_vertex_attrib_array(vertex_color);

  // Tell WebGL to use our program when drawing
  context.use_program(Some(&program));

  // Set the shader uniforms
  let data: JsValue = JsValue::from_serde(&projection_matrix).unwrap().into();
  context.uniform_matrix4fv_with_f32_sequence(
      unf_projection_matrix, false, &data
  );
  let data: JsValue = JsValue::from_serde(&model_view_matrix).unwrap().into();
  context.uniform_matrix4fv_with_f32_sequence(
      unf_model_view_matrix, false, &data
  );

  context.draw_arrays(
      WebGlRenderingContext::TRIANGLE_STRIP,
      0,
      (4) as i32,
  );

  Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
      .create_shader(shader_type)
      .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
      .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| "Unknown error creating shader".into()))
  }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGlRenderingContext,
    shaders: T,
) -> Result<WebGlProgram, String> {
  let program = context
      .create_program()
      .ok_or_else(|| String::from("Unable to create shader object"))?;
  for shader in shaders {
    context.attach_shader(&program, shader)
  }
  context.link_program(&program);

  if context
      .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(context
        .get_program_info_log(&program)
        .unwrap_or_else(|| "Unknown error creating program object".into()))
  }
}
