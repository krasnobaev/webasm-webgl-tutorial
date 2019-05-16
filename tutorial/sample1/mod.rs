extern crate wasm_bindgen;
extern crate web_sys;
extern crate rustfft;
extern crate serde_derive;
extern crate js_sys;

// use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
  // console,
  // WebGlProgram,
  WebGlRenderingContext,
  // WebGlShader,
};

// use js_sys::WebAssembly;

pub fn draw (
  context: &WebGlRenderingContext
) -> Result<(), JsValue> {

  // @see sample1 in webgl tutorial
  {
    // Set clear color to black, fully opaque
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    // Clear the color buffer with specified clear color
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
  }

  Ok(())
}
