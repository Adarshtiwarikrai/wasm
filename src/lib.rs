use js_sys::Math::abs;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap as Hashmap;
use std::f64;
use std::rc::Rc;
use std::sync::{OnceLock, Mutex};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;
mod class;
use class::Shape;
mod state;
use state::State;
use web_sys::{
    console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
    WebGlProgram, WebGlRenderingContext, WebGlShader,
};


pub fn draw(context: CanvasRenderingContext2d, shapes: &Vec<Shape>,moves:&Vec<Shape>) {
    context.clear_rect(0.0, 0.0, 10000.0, 10000.0); 
    for shape in shapes.iter() {
        shape.create_shape(context.clone());
    }
    for movee in moves.iter() {
        movee.create_shape(context.clone());
    }
}
static STATE: OnceLock<Mutex<State>> = OnceLock::new();
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    
    console::log_1(&"Hello from Rust WASM! this is first ".into());

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
    let circle = document
        .get_element_by_id("circle")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;
    let line = document
        .get_element_by_id("draw")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;
    let arrow = document
        .get_element_by_id("arrow")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let undobutton = document
        .get_element_by_id("undo")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;

    let redo = document
        .get_element_by_id("redo")
        .ok_or("No canvas found")?
        .dyn_into::<HtmlElement>()?;

    console_error_panic_hook::set_once();

    // -------------------------------------------------------------------
    // square click
    // -------------------------------------------------------------------
    {   
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
          let state = STATE.get().expect("GameState not initialized!");
          let mut s=state.lock().unwrap();
          console::log_1(&"square action".into());
          s.square=!s.square;
         
        }) as Box<dyn FnMut(_)>);

        square.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
      // -------------------------------------------------------------------
    // circle click
    // -------------------------------------------------------------------
    {   
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
          let state = STATE.get().expect("GameState not initialized!");
          let mut s=state.lock().unwrap();
         s.circle=!s.circle;
        }) as Box<dyn FnMut(_)>);

        circle.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
      // -------------------------------------------------------------------
    // arrow click
    // -------------------------------------------------------------------
    {    
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
          let state = STATE.get().expect("GameState not initialized!");
          let mut s=state.lock().unwrap();
          s.arrow=!s.arrow;
        }) as Box<dyn FnMut(_)>);

        arrow.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
      // -------------------------------------------------------------------
    // draw click
    // -------------------------------------------------------------------
    {    
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
          let state = STATE.get().expect("GameState not initialized!");
          let mut s=state.lock().unwrap();
         s.draw=!s.draw;
        }) as Box<dyn FnMut(_)>);

        line.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }


    // -------------------------------------------------------------------
    // canvas mousedown -- add shape
    // -------------------------------------------------------------------
    {
        
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let state = STATE.get().expect("GameState not initialized!");
            let mut s=state.lock().unwrap();
            s.mousedown(new_x,new_y);
            
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousedown", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    {    
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {

            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            console::log_1(&"mouse is move arrow ".into());
            let state = STATE.get_or_init(|| {
                console::log_1(&"mouse is move in function arrow ".into());
                println!("Initializing GameState...");
                Mutex::new(State::new(new_x, new_y))
            });
        
            let mut s=state.lock().unwrap();
            s.mousemove(new_x,new_y);
            if s.square==true {
                s.removemoveshape();
                s.shape("square".to_string());
            }
            else if s.circle==true{
                s.removemoveshape();
                s.shape("circle".to_string());
            }
            else if s.arrow==true{
                s.removemoveshape();
                s.shape("arrow".to_string());
            }
            else if s.draw==true{
                s.removemoveshape();
                s.shape("draw".to_string());
            }
            draw(context.clone(), &s.shapes,&s.movesshapes);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    {

        
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let state = STATE.get().expect("GameState not initialized!");
            let mut s=state.lock().unwrap();
            s.mouseup(new_x,new_y);
            if s.square==true {
                s.movetoshape();
                s.square=false;
            }
            else if s.circle==true{
                s.movetoshape();
                s.circle=false;
            }
            else if s.arrow==true{
                s.movetoshape();
                s.arrow=false;
            }
            else if s.draw==true{
                s.movetoshape();
                s.draw=false;
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup",click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    // -------------------------------------------------------------------
    // redo button
    // -------------------------------------------------------------------
    {
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
            let state = STATE.get().expect("GameState not initialized!");
            let mut s=state.lock().unwrap();
            s.redo();
            draw(context.clone(), &s.shapes,&s.movesshapes);
        }) as Box<dyn FnMut(_)>);

        redo.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }

    // -------------------------------------------------------------------
    // undo button
    // -------------------------------------------------------------------
    {
        let context=context.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
            let state = STATE.get().expect("GameState not initialized!");
            let mut s=state.lock().unwrap();
            s.undo(); 
            draw(context.clone(), &s.shapes,&s.movesshapes);
        }) as Box<dyn FnMut(_)>);

        undobutton.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }

    Ok(())
}
