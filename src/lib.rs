use js_sys::Math::abs;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap as Hashmap;
use std::f64;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;
mod class;
use class::Shape;
use web_sys::{
    console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
    WebGlProgram, WebGlRenderingContext, WebGlShader,
};

lazy_static! {
    static ref buttonclick: Mutex<bool> = Mutex::new(false);
}
lazy_static! {
    static ref x: Mutex<f64> = Mutex::new(0.0);
}
lazy_static! {
    static ref y: Mutex<f64> = Mutex::new(0.0);
}
pub fn draw(context: CanvasRenderingContext2d, shapes: &Vec<Shape>,moves:&Vec<Shape>) {
    context.clear_rect(0.0, 0.0, 10000.0, 10000.0); 
    for shape in shapes.iter() {
        shape.create_shape(context.clone());
    }
    for movee in moves.iter() {
        movee.create_shape(context.clone());
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let shapes = Rc::new(RefCell::new(Vec::new()));
    let moveshapes=Rc::new(RefCell::new(Vec::new()));
    let undoshapes = Rc::new(RefCell::new(Vec::new()));

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
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
            let mut num = buttonclick.lock().unwrap();
            *num = !*num;
            console::log_2(&"the click happened".into(), &JsValue::from_bool(*num));
        }) as Box<dyn FnMut(_)>);

        square.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }

    // -------------------------------------------------------------------
    // canvas mousedown -- add shape
    // -------------------------------------------------------------------
    {
        let context = context.clone();
        let shapes_ref = shapes.clone();
        let undo_ref = undoshapes.clone();
        let moves_ref=moveshapes.clone();
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let mut num = x.lock().unwrap();
            *num = new_x;
            let mut num1 = y.lock().unwrap();
            *num1 = new_y;
            let shape = Shape::new(
                new_x,
                new_y,
                new_x+10.0,
                new_y+10.0,
                0.0,
                360.0,
                10.0,
                "square".to_string(),
                "red".to_string(),
                false,
                0.0,
                0.0,
                1.0,
                1.0,
            );

            shapes_ref.borrow_mut().push(shape);
            undo_ref.borrow_mut().clear();
            draw(context.clone(), &shapes_ref.borrow(),&moves_ref.borrow());
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousedown", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    {
        let context = context.clone();
        let moves_ref=moveshapes.clone();
        let shapes_ref= shapes.clone();
        let undo_ref=undoshapes.clone();
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let mut num = x.lock().unwrap();
            let mut num1 = y.lock().unwrap();
            let shape = Shape::new(
                *num,
                *num1,
                new_x,
                new_y,
                0.0,
                360.0,
                10.0,
                "square".to_string(),
                "red".to_string(),
                false,
                0.0,
                0.0,
                1.0,
                1.0,
            );
            console::log_2(&JsValue::from_f64(new_x), &JsValue::from_f64(new_y));

            {
                let mut vec = moves_ref.borrow_mut();
                vec.clear();
                vec.push(shape);
            } 
            draw(context.clone(), &moves_ref.borrow(),&shapes_ref.borrow());
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    {

        let context=context.clone();
        let moves_ref=moveshapes.clone();
        let shapes_ref= shapes.clone();
        
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
        if moves_ref.borrow().len()>0{
           if let Some(last)=moves_ref.borrow_mut().pop(){
            shapes_ref.borrow_mut().push(last);
           }
           moves_ref.borrow_mut().clear();
        }
        
           
            draw(context.clone(), &shapes_ref.borrow(),&moves_ref.borrow());
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup",click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }
    // -------------------------------------------------------------------
    // redo button
    // -------------------------------------------------------------------
    {
        let context = context.clone();
        let shapes_ref = shapes.clone();
        let undo_ref = undoshapes.clone();
        let moves_ref=moveshapes.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
            console::log_1(&"redo button clicked".into());

            let undo_len = undo_ref.borrow().len();
            console::log_1(&JsValue::from(undo_len));

            if undo_len > 0 {
                console::log_1(&"redo button clicked hi from inside ".into());

                if let Some(last) = undo_ref.borrow_mut().pop() {
                    shapes_ref.borrow_mut().push(last);
                }
                let undo_len = undo_ref.borrow().len();
                console::log_1(&JsValue::from(undo_len));
                console::log_1(&"redo button clicked hi from outside ".into());

                draw(context.clone(), &shapes_ref.borrow(),&moves_ref.borrow());
            }
        }) as Box<dyn FnMut(_)>);

        redo.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }

    // -------------------------------------------------------------------
    // undo button
    // -------------------------------------------------------------------
    {
        let context = context.clone();
        let shapes_ref = shapes.clone();
        let undo_ref = undoshapes.clone();
        let moves_ref=moveshapes.clone();
        let click_down = Closure::wrap(Box::new(move |_event: MouseEvent| {
            console::log_1(&"undo button clicked".into());

            let shape_len = shapes_ref.borrow().len();
            console::log_1(&JsValue::from(shape_len));

            if shape_len > 0 {
                console::log_1(&"undo button clicked hi from inside ".into());
                if let Some(last) = shapes_ref.borrow_mut().pop() {
                    undo_ref.borrow_mut().push(last);
                }
                let shape_len = shapes_ref.borrow().len();
                console::log_1(&JsValue::from(shape_len));
                console::log_1(&"undo button clicked work done  ".into());

                draw(context.clone(), &shapes_ref.borrow(),&moves_ref.borrow());
            }
        }) as Box<dyn FnMut(_)>);

        undobutton.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget();
    }

    Ok(())
}
