#[macro_use]
pub mod macros;
pub mod graphics;
pub mod sprite;

use stdweb::{ Reference };
use stdweb::unstable::{ TryFrom };
use stdweb::web::{
    HtmlElement,
};

pub trait JsRef {
    fn js_ref(&self) -> &Reference;
}

pub trait Positionable {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn set_x(&self, x: f64);
    fn set_y(&self, y: f64);
    fn add_x(&self, n: f64) {
        self.set_x(self.x() + n);
    }
    fn add_y(&self, n: f64) {
        self.set_y(self.y() + n);
    }
    fn set_position(&self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
}

pub trait Sizable {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn set_width(&self, width: f64);
    fn set_height(&self, height: f64);
    fn set_size(&self, width: f64, height: f64) {
        self.set_width(width);
        self.set_height(height);
    }
    fn set_square_size(&self, size: f64) {
        self.set_size(size, size);
    }
}

pub trait Rotatable {
    fn angle(&self) -> f64;
    fn set_angle(&self, angle: f64);
    fn add_angle(&self, angle: f64) {
        self.set_angle(self.angle() + angle);
    }
}

pub struct Pixi(Reference);

impl JsRef for Pixi {
    fn js_ref(&self) -> &Reference {
        &self.0
    }
}

impl Pixi {
    pub fn new(width: u32, height: u32, background_color: u32) -> Self {
        let app = js! {
            return new PIXI.Application ({
                width: @{width},
                height: @{height},
                backgroundColor: @{background_color},
            });
        };

        Pixi(app.into_reference().unwrap())
    }

    pub fn view(&self) -> HtmlElement {
        let view = js! {
            const app = @{&self.js_ref()};
            return app.view;
        };
        
        HtmlElement::try_from(view).unwrap()
    }

    pub fn add_child(&self, child: &impl JsRef) {
        js! { @(no_return)
            const app = @{&self.js_ref()};
            app.stage.addChild(@{&child.js_ref()});
        };
    }

    pub fn add_child_at(&self, child: &impl JsRef, index: u32) {
        js! { @(no_return)
            const app = @{&self.js_ref()};
            app.stage.addChildAt(@{&child.js_ref()}, @{index});
        };
    }

    pub fn remove_child(&self, child: &impl JsRef) {
        js! { @(no_return)
            const app = @{&self.js_ref()};
            app.stage.removeChild(@{&child.js_ref()});
        };
    }
}
