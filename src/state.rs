use js_sys::Math::abs;
use web_sys::{
  console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
  WebGlProgram, WebGlRenderingContext, WebGlShader,
};
mod class;
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
    pub actions:String
}
impl State {
  pub fn new(
    startx:f64,
    starty:f64
  )->self{
    State{
        startx,
        starty,
        startx,
        starty,
        startx,
        starty,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        "mousemove"
    }
  }
  pub fn mousedown(&self ,startx:f64,starty:f64){
    self.mousedownx=startx;
    self.mousedowny=starty;
  }
  pub fn mousemove(&self,startx:f64,starty:f64){
    self.mousemovex=startx;
    self.mousemovey=starty;
  }
  pub fn mouseup(&self,startx:f64,starty:f64){
    self.mouseupx=startx;
    self.mouseupy=starty;
  }
  pub fn undo(&self){
    if self.shapes.len()>0{
      if let Some(last)=shapes.pop(){
        self.undoshapes.push(last);
      }
    }
  }
  pub fn redo(&self){
    if self.undoshapes.len()>0{
        if let Some(last)=undoshapes.pop(){
            self.shapes.push(last);
        }
    }
  }
  pub fn shape(&self,action:String){
    if action=="arrow" {
      let shape=Shape::new(
        self.mousedownx,
        self.mousedowny,
        self.mouseupx,
        self.mouseupy,
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
    self.shapes.push(shape);
    }
    else if action=="square"{
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mouseupx,
            self.mouseupy,
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
        self.shapes.push(shape);
    }
    else if action=="circle"{
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mouseupx,
            self.mouseupy,
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
        self.shapes.push(shape);
    }
    else if action=="line"{
        let shape=Shape::new(
            self.mousedownx,
            self.mousedowny,
            self.mouseupx,
            self.mouseupy,
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
        self.shapes.push(shape);
    }

  }
}
