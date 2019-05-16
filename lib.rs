extern crate wasm_bindgen;
extern crate web_sys;
extern crate rustfft;
extern crate serde_derive;
extern crate js_sys;
extern crate nalgebra_glm as glm;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
  // console,
  // WebGlProgram,
  WebGlRenderingContext,
  // WebGlShader
};

// use js_sys::WebAssembly;

mod tutorial;

/* Web GL */

#[wasm_bindgen]
pub fn drawwebgl(data: &JsValue) -> Result<(), JsValue> {
  // let dat2: Vec<u8> = data.into_serde().unwrap();
  let sample_id: u8 = data.into_serde().unwrap();

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  let context = canvas
      .get_context("webgl")?
      .unwrap()
      .dyn_into::<WebGlRenderingContext>()?;

  match sample_id {
    1 => tutorial::sample1::draw(&context)?,
    2 => tutorial::sample2::draw(&context, canvas.width() as f32, canvas.height() as f32)?,
    3 => tutorial::sample3::draw(&context, canvas.width() as f32, canvas.height() as f32)?,
    4 => tutorial::sample4::draw(&context, canvas.width() as f32, canvas.height() as f32)?,
    5 => tutorial::sample5::draw(&context, canvas.width() as f32, canvas.height() as f32)?,
    _ => (),
  }

  Ok(())
}
