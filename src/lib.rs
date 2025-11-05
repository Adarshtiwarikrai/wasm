mod class;
use class::Shape;
use js_sys::Math::abs;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use web_sys::HtmlElement;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
    WebGlProgram, WebGlRenderingContext, WebGlShader,
};
lazy_static! {
    static ref vec: Mutex<Vec<Shape>>=Mutex::new(Vec::new());
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>{
    let window = web_sys::window().ok_or("No global window found")?;
    let document = window.document().ok_or("No document found")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlCanvasElement>()?;
    let square = document
        .get_element_by_id("square")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;
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
    }
    Ok(())
}
