use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable, Sizable, Rotatable };

#[derive(Clone, PartialEq, Debug)]
pub struct Sprite(Reference);

impl Sprite {
    pub fn new(alias: &str) -> Self {
        let sprite = js! {
            const sprite = new PIXI.Sprite(PIXI.loader.resources[@{alias}].texture);
            sprite.anchor.x = 0.5;
            sprite.anchor.y = 0.5;
            return sprite;
        };
        
        Sprite(sprite.into_reference().unwrap())
    }
}

impl JsRef for Sprite {
    fn js_ref(&self) -> &Reference {
        &self.0
    }
}

impl Positionable for Sprite {
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

impl Sizable for Sprite {
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

impl Rotatable for Sprite {
    fn angle(&self) -> f64 {
        js_get!(self, rotation)
    }

    fn set_angle(&self, angle: f64) {
        js_set!(self, rotation, angle);
    }
}