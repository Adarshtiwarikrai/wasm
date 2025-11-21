use js_sys::Math::abs;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
  console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
  WebGlProgram, WebGlRenderingContext, WebGlShader,
};
use crate::class;
use class::Shape;
#[derive(Debug, Clone)]
pub struct State {
    pub mousedownx:f64,
    pub mousedowny:f64,
    pub mousemovex:f64,
    pub mousemovey:f64,
    pub mouseupx:f64,
    pub mouseupy:f64,
    pub shapes:Vec<Shape>,
    pub undoshapes:Vec<Shape>,
    pub redoshapes:Vec<Shape>,
    pub movesshapes:Vec<Shape>,
    pub actions:String,
    pub square:bool,
    pub circle:bool,
    pub arrow:bool,
    pub draw:bool
}
impl State {
  pub fn new(
    startx:f64,
    starty:f64
  )->Self{
    State{
        mousedownx:startx,
        mousedowny:starty,
        mousemovex:startx,
        mousemovey:starty,
        mouseupx:startx,
        mouseupy:starty,
        shapes:Vec::new(),
        undoshapes:Vec::new(),
        redoshapes:Vec::new(),
        movesshapes:Vec::new(),
        actions:"".to_string(),
        square:false,
        circle:false,
        arrow:false,
        draw:false
    }
  }
  pub fn mousedown(&mut self ,startx:f64,starty:f64){
    self.mousedownx=startx;
    self.mousedowny=starty;
  }
  pub fn mousemove(&mut self,startx:f64,starty:f64){
    self.mousemovex=startx;
    self.mousemovey=starty;
  }
  pub fn mouseup(&mut self,startx:f64,starty:f64){
    self.mouseupx=startx;
    self.mouseupy=starty;
  }
  pub fn undo(& mut self){
    if self.shapes.len()>0{
      if let Some(last)=self.shapes.pop(){
        self.undoshapes.push(last);
      }
    }
  }
  pub fn redo(& mut self){
    if self.undoshapes.len()>0{
        if let Some(last)=self.undoshapes.pop(){
            self.shapes.push(last);
        }
    }
  } 
  pub fn movetoshape(& mut self){
    if self.movesshapes.len()>0{
      if let Some(last)=self.movesshapes.pop(){
          self.shapes.push(last);
      }
  }
  }
  pub fn removemoveshape(& mut self){
    if self.movesshapes.len()>0{
      if let Some(last)=self.movesshapes.pop(){
          
      }
  }
  }
  pub fn shape(& mut self,action:String){
    if action=="arrow" {
      let shape=Shape::new(
        self.mousedownx,
        self.mousedowny,
        self.mousemovex,
        self.mousemovey,
        0.0,
        360.0,
        10.0,
        "arrow".to_string(),
        "red".to_string(),
        false,
        0.0,
        0.0,
        1.0,
        1.0,
    );
    self.movesshapes.push(shape);
   
    }
    else if action=="square"{
      console::log_1(&"square action in state ".into());
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mousemovex,
            self.mousemovey,
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
        self.movesshapes.push(shape);
        console::log_1(&JsValue::from(self.shapes.len()));
    }
    else if action=="circle"{
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mousemovex,
            self.mousemovey,
            0.0,
            360.0,
            10.0,
            "circle".to_string(),
            "red".to_string(),
            false,
            0.0,
            0.0,
            1.0,
            1.0,
        );
        self.movesshapes.push(shape);
    }
    else if action=="line"{
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mousemovex,
            self.mousemovey,
            0.0,
            360.0,
            10.0,
            "line".to_string(),
            "red".to_string(),
            false,
            0.0,
            0.0,
            1.0,
            1.0,
        );
        self.movesshapes.push(shape);
    }

  }
}
