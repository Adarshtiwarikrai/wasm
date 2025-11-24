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
    pub dragshapes:Vec<Shape>,
    pub actions:String,
    pub square:bool,
    pub circle:bool,
    pub arrow:bool,
    pub draw:bool,
    pub drag:bool,
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
        dragshapes:Vec::new(),
        actions:"".to_string(),
        square:false,
        circle:false,
        arrow:false,
        draw:false,
        drag:false,
    }
  }
  pub fn shapeview(&mut self,context:CanvasRenderingContext2d,types:String){
    context.clear_rect(0.0, 0.0, 10000.0, 10000.0); 
    for shape in self.shapes.iter() {
      shape.create_shape(context.clone());
     }
    for shape in self.dragshapes.iter() {
    shape.create_shape(context.clone());
     } 
     for shape in self.movesshapes.iter() {
  shape.create_shape(context.clone());
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
  pub fn removemoveshape(& mut self,context:CanvasRenderingContext2d){
    if self.movesshapes.len()>0{
      if let Some(last)=self.movesshapes.pop(){
        let lw = context.line_width();
        context.clear_rect(last.startx-lw/2.0, last.starty-lw/2.0, last.length+1.0, last.width+1.0);   
      }
  }
  }
  pub fn movetodragshapes(& mut self,startx:f64,starty:f64){
    
    for i in  (0..self.shapes.len()).rev() {
      let shape = &self.shapes[i]; 

      if shape.shape=="square"{
          let len=(shape.endx-shape.startx) as f64;
          let len2=(shape.endy-shape.starty) as f64;
          let inside_x = shape.startx <= startx && shape.startx+len >= startx;
          let inside_y = shape.starty <= starty && shape.starty+len2 >= starty;

            if inside_x || inside_y {
              console::log_1(&"square clicked in boundary ".into());
            self.drag=true;
            let item=self.shapes.remove(i);
            self.dragshapes.push(item);
            }
      }
    }
  
  }
  pub fn checktodragshapes(& mut self,startx:f64,starty:f64,canvas:HtmlCanvasElement){
    
    for i in  (0..self.shapes.len()).rev() {
      let shape = &self.shapes[i]; 

      if shape.shape=="square"{
          let len=(shape.endx-shape.startx) as f64;
          let len2=(shape.endy-shape.starty) as f64;
          let inside_x = shape.startx <= startx && shape.startx+len >= startx;
          let inside_y = shape.starty <= starty && shape.starty+len2 >= starty;

            if inside_x || inside_y {
              console::log_1(&"square clicked in boundary ".into());
             
            }
      }
    }
  
  }
  pub fn removedragshape(& mut self,context:CanvasRenderingContext2d,startx:f64,starty:f64){
    if let Some(mut shape) = self.dragshapes.pop() {
      console::log_1(&"square moving ".into());
      let lw = context.line_width();
      context.clear_rect(shape.startx-lw/2.0, shape.starty-lw/2.0, shape.length+1.0, shape.width+1.0); 
      shape.startx = startx;
      shape.starty = starty;
      
      self.dragshapes.push(shape);
    }
  }
  pub fn dragtoshape(& mut self){
    if self.dragshapes.len()>0{
      console::log_1(&"square to  the shape from drag ".into());
      if let Some(last)=self.dragshapes.pop(){
          self.shapes.push(last);
      }
  }
  }
  pub fn shape(& mut self,action:String){
    if action=="arrow" {
      let mut  shape=Shape::new(
        self.mousedownx,
        self.mousedowny,
        self.mousemovex,
        self.mousemovey,
        0.0,
        360.0,
        0.0,
        0.0,
        10.0,
        "arrow".to_string(),
        "red".to_string(),
        false,
        0.0,
        0.0,
        1.0,
        1.0,
    );
    shape.calculate();
    self.movesshapes.push(shape);
   
    }
    else if action=="square"{
     
        let mut shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mousemovex, 
            self.mousemovey,
            0.0,
            360.0,
            0.0,
            0.0,
            10.0,
            "square".to_string(),
            "red".to_string(),
            false,
            0.0,
            0.0,
            1.0,
            1.0,
        );
        shape.calculate();
        self.movesshapes.push(shape);
        console::log_1(&JsValue::from(self.shapes.len()));
    }
    else if action=="circle"{
        let mut  shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mousemovex,
            self.mousemovey,
            0.0,
            360.0,
            0.0,
            0.0,
            10.0,
            "circle".to_string(),
            "red".to_string(),
            false,
            0.0,
            0.0,
            1.0,
            1.0,
        );
        shape.calculate();
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
            0.0,
            0.0,
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
