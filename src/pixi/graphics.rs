use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable, Sizable, Rotatable };

#[derive(Clone, PartialEq, Debug, Positionable, Sizable, Rotatable)]
pub struct Graphics(Reference);

impl Graphics {
    pub fn new() -> Self {
        let graphics = js! {
            return new PIXI.Graphics();
        };
        
        Graphics(graphics.into_reference().unwrap())
    }

    pub fn line_width(&self, width: u32) {
        js_set!(self, lineWidth, width);
    }

    pub fn line_color(&self, color: u32) {
        js_set!(self, lineColor, color);
    }

    pub fn begin_fill(&self, color: u32) {
        js_function!(self, beginFill, color);
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: u32, height: u32) {
        js_function!(self, drawRect, x, y, width, height);
    }

    pub fn draw_ellipse(&self, x: f64, y: f64, width: u32, height: u32) {
        js_function!(self, drawEllipse, x, y, width, height);
    }
}

impl JsRef for Graphics {
    fn js_ref(&self) -> &Reference {
        &self.0
    }
}