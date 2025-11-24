use wasm_bindgen::{prelude::*, JsCast};

use lazy_static::lazy_static;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use js_sys::Math::abs;
use std::sync::Mutex;
use web_sys::HtmlElement;
use web_sys::{
    console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
    WebGlProgram, WebGlRenderingContext, WebGlShader,
};
lazy_static! {
    static ref x: Mutex<f64> = Mutex::new(0.0);
}
lazy_static! {
    static ref y: Mutex<f64> = Mutex::new(0.0);
}
lazy_static! {
    static ref word: Mutex<String> = Mutex::new(String::new());
}
lazy_static! {
    static ref buttonclick: Mutex<bool> = Mutex::new(false);
}
lazy_static! {
    static ref mouseclick: Mutex<bool> = Mutex::new(false);
}
lazy_static! {
    static ref movex: Mutex<f64> = Mutex::new(0.0);
}
lazy_static! {
    static ref movey: Mutex<f64> = Mutex::new(0.0);
}
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Access the document and canvas
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
        let context2 = context.clone();
        console::log_1(&"Hello from mouse event 1".into());
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            console::log_1(&"Hello from mouse event 2".into());
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let mut num = buttonclick.lock().unwrap();
            let mut num2 = mouseclick.lock().unwrap();
            if (*num == true) {
                *num2 = true;
                console::log_2(&JsValue::from_bool(*num), &JsValue::from_bool(*num2));
            } 
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
            let mut num = x.lock().unwrap();
            *num = new_x;
            let mut num1 = y.lock().unwrap();
            *num1 = new_y;
            console::log_1(&"Hello from mouse event 3".into());
            console::log_2(&JsValue::from_f64(new_x), &JsValue::from_f64(new_y));
        }) as Box<dyn FnMut(_)>);

        let key_down = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            console::log_1(&"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into());
            console::log_1(&"Hello from keyboard event 2".into());
            let key = event.key();
            let mut num = x.lock().unwrap();
            let mut num1 = y.lock().unwrap();
            let mut num2 = word.lock().unwrap();
            *num2 += &key;
            context2.set_font("20px Georgia");
            context2.stroke_text(&num2, *num as f64, *num1 as f64);
            console::log_1(&key.into());
            console::log_1(&"Hello from keyboard event 2".into());

            console::log_1(&"Hello from keyboard event 3".into());
        }) as Box<dyn FnMut(_)>);
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            console::log_1(&"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into());
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let mut num = x.lock().unwrap();
            *num = new_x;
            let mut num1 = y.lock().unwrap();
            *num1 = new_y;
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())?;

        document.add_event_listener_with_callback("keydown", key_down.as_ref().unchecked_ref())?;
       

        mouse_down.forget();
        key_down.forget();
    }
    {
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut num = buttonclick.lock().unwrap();
            *num = !*num;
            console::log_2(&"the click is happen".into(), &JsValue::from_bool(*num));
        }) as Box<dyn FnMut(_)>);
        square.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        circle.add_event_listener_with_callback("click", click_down.as_ref().unchecked_ref())?;
        click_down.forget()
    }
    {
        let context = context.clone();
        let click_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            let new_x = event.offset_x() as f64;
            let new_y = event.offset_y() as f64;
            let mut num = x.lock().unwrap();
            let mut num1 = y.lock().unwrap();
            let mut lastx = movex.lock().unwrap();
            let mut lasty = movey.lock().unwrap();

            // context.set_stroke_style_str("blue");
            //  context.stroke_rect(event.offset_x() as f64, event.offset_y() as f64, 10 as f64, 30 as f64);
            let mut button = buttonclick.lock().unwrap();
            let mut mouse = mouseclick.lock().unwrap();
            let mut lastnum=*num;
            let mut lastnum1=*num1;
            if (*button == true && *mouse == true) {
                 console::log_3(&"before".into(),&JsValue::from_f64(*num), &JsValue::from_f64(*num1));
                 if *lastx < 0.0 {
                    *num = *num + 1.0;
                 }
                 else {
                    *num = *num - 1.0;  
                 }
                 if *lasty < 0.0 {
                    *num1 = *num + 1.0;
                 }
                 else {
                    *num1 = *num - 1.0;  
                 }
                context.clear_rect(
                    *num-1 as f64,
                    *num1-1 as f64,
                    (*lastx) as f64,
                    (*lasty)  as f64,
                );
                context.set_stroke_style_str("blue");
                console::log_3(&"after".into(),&JsValue::from_f64(*num), &JsValue::from_f64(*num1));
                context.stroke_rect(
                    *num as f64,
                    *num1 as f64,
                    (new_x-*num)  as f64,
                    (new_y-*num1)  as f64,
                );
                

                *lastx =(new_x-*num)  ;
                *lasty = (new_y-*num1)  ;

                if *lastx < 0.0 {
                    *lastx += -2.0;
                }
                else {
                    *lastx += 2.0;
                }
                if *lasty < 0.0 {
                    *lasty += -2.0;
                }
                else {
                    *lasty += 2.0;
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", click_down.as_ref().unchecked_ref())?;
        click_down.forget()
    }
    {
        let context = context.clone();
        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut num = x.lock().unwrap();
            let mut num1 = y.lock().unwrap();
            *num = 0.0;
            *num1 = 0.0;
            // context.set_stroke_style_str("blue");
            //  context.stroke_rect(event.offset_x() as f64, event.offset_y() as f64, 10 as f64, 30 as f64);
            let mut button = buttonclick.lock().unwrap();
            let mut mouse = mouseclick.lock().unwrap();
            if (*button == true && *mouse == true) {
                *mouse = false;
                *button = false;
                let mut lastx = movex.lock().unwrap();
                let mut lasty = movey.lock().unwrap();
                *lastx = 0.0;
                *lasty = 0.0;
                console::log_2(&"mouse is up ".into(), &JsValue::from_bool(*mouse));
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())?;
        mouse_up.forget()
    }

    Ok(())
}

// cargo build --target wasm32-unknown-unknown
// wasm-bindgen target/wasm32-unknown-unknown/debug/rust_webgl_example.wasm \
//   --out-dir pkg --target web
// asdfasdfasdfassdfasdfadsfa
//python -m http.server 8000
// wasm-bindgen target/wasm32-unknown-unknown/debug/rust_webgl_example.wasm  --out-dir pkg --target web
