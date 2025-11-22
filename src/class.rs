use js_sys::Math::abs;
use web_sys::{
  console, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent,
  WebGlProgram, WebGlRenderingContext, WebGlShader,
};
#[derive(Debug, Clone)]
pub struct Shape {
    pub startx: f64,
    pub starty: f64,
    pub endx: f64,
    pub endy: f64,
    pub startangle: f64,
    pub endangle: f64,
    pub radius: f64,
    pub shape: String,
    pub colour: String,
    pub isdash: bool,
    pub dashlength: f64,
    pub dashspace: f64,
    pub stokewidth: f64,
    pub rotate: f64,
}
impl Shape {
  pub fn new(
      startx: f64,
      starty: f64,
      endx: f64,
      endy: f64,
      startangle: f64,
      endangle: f64,
      radius: f64,
      shape: String,
      colour: String,
      isdash: bool,
      dashlength: f64,
      dashspace: f64,
      stokewidth: f64,
      rotate: f64,
  ) -> Self {
      Shape {
          startx,
          starty,
          endx,
          endy,
          startangle,
          endangle,
          radius,
          shape,
          colour,
          isdash,
          dashlength,
          dashspace,
          stokewidth,
          rotate,
      }
  }

  pub fn create_shape(&self, context: CanvasRenderingContext2d) {
      if self.shape == "square" {
        console::log_1(&"square action in shape".into());
          context.stroke_rect(self.startx, self.starty, (self.endx-self.startx), (self.endy-self.starty));
      } 
      else if self.shape == "circle" {
        
          context.begin_path();
          context
              .ellipse(
                  self.startx,
                  self.starty,
                  abs(self.endx-self.startx) as f64,
                  abs(self.endy-self.starty) as f64,
                  90.0,
                  0.0,
                  360.0,
              )
              .unwrap();
              context.set_stroke_style_str(&self.colour );
          context.stroke();
      } 
      else if self.shape == "line" {
          context.begin_path();
          context.line_to(self.endx, self.endy);
          context.set_stroke_style_str(&self.colour );
          context.stroke();
      } 
      else if self.shape == "arrow" {
          context.begin_path();
          context.move_to(self.startx, self.starty);
          context.line_to(self.endx, self.endy);
          context.set_stroke_style_str(&self.colour );
          context.stroke();
      }
  }
}
