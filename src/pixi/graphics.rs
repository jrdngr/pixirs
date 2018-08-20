use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable };

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
        js! { @(no_return)
            const graphics = @{&self.js_ref()};
            graphics.lineWidth = @{width};
        };
    }

    pub fn line_color(&self, color: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_ref()};
            graphics.lineColor = @{color};
        };
    }

    pub fn begin_fill(&self, color: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_ref()};
            graphics.beginFill(@{color});
        };
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: u32, height: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_ref()};
            graphics.drawRect(@{x}, @{y}, @{width}, @{height});
        };
    }

    pub fn draw_ellipse(&self, x: f64, y: f64, width: u32, height: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_ref()};
            graphics.drawEllipse(@{x}, @{y}, @{width}, @{height});
        };
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
        let y = js! { 
            const me = @{&self.js_ref()};
            return me.y;
        };
        y.try_into().unwrap()
    }

    fn set_y(&self, new_y: f64) {
        js! { @(no_return)
            const rect = @{&self.js_ref()};
            rect.y = @{new_y};
        };
    }

    fn set_position(&self, x: f64, y: f64) {
        js! {@(no_return)
            const rect = @{&self.js_ref()};
            rect.x = @{x};
            rect.y = @{y};
        };
    }
}