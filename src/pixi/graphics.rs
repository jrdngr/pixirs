use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable, Sizable, Rotatable };

#[derive(Clone, PartialEq, Debug)]
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

impl Positionable for Graphics {
    fn x(&self) -> f64 {
        js_get!(self, x)
    }

    fn set_x(&self, new_x: f64) {
        js_set!(self, x, new_x);
    }

    fn y(&self) -> f64 {
        js_get!(self, y)
    }

    fn set_y(&self, new_y: f64) {
        js_set!(self, y, new_y);
    }
}

impl Sizable for Graphics {
    fn width(&self) -> f64 {
        js_get!(self, width)
    }

    fn set_width(&self, new_width: f64) {
        js_set!(self, width, new_width);
    }

    fn height(&self) -> f64 {
        js_get!(self, height)
    }

    fn set_height(&self, new_height: f64) {
        js_set!(self, height, new_height);
    }
}

impl Rotatable for Graphics {
    fn angle(&self) -> f64 {
        js_get!(self, rotation)
    }

    fn set_angle(&self, angle: f64) {
        js_set!(self, rotation, angle);
    }
}