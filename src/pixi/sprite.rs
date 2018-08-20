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
        let x = js! { 
            const me = @{&self.js_ref()};
            return me.x;
        };
        x.try_into().unwrap()
    }

    fn set_x(&self, new_x: f64) {
        js! { @(no_return)
            const rect = @{&self.js_ref()};
            rect.x = @{new_x};
        };
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

impl Sizable for Sprite {
    fn width(&self) -> f64 {
        let x = js! { 
            const me = @{&self.js_ref()};
            return me.width;
        };
        x.try_into().unwrap()
    }

    fn set_width(&self, width: f64) {
        js! { @(no_return)
            const rect = @{&self.js_ref()};
            rect.width = @{width};
        };
    }

    fn height(&self) -> f64 {
        let y = js! { 
            const me = @{&self.js_ref()};
            return me.height;
        };
        y.try_into().unwrap()
    }

    fn set_height(&self, height: f64) {
        js! { @(no_return)
            const rect = @{&self.js_ref()};
            rect.height = @{height};
        };
    }
}

impl Rotatable for Sprite {
    fn angle(&self) -> f64 {
        let angle = js! {
            const me = @{&self.js_ref()};
            return me.rotation;
        };
        angle.try_into().unwrap()
    }

    fn set_angle(&self, angle: f64) {
        js! { @(no_return)
            const me = @{&self.js_ref()};
            me.rotation = @{angle};
        };
    }
}