#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

pub mod pixi;

use stdweb::traits::*;
use pixi::{Positionable, Sizable, Rotatable};

fn main() {
    let body = stdweb::web::document().body().unwrap();

    let mut pixi = pixi::Pixi::new(1024, 768, 0x000000);
    let mut square = pixi::graphics::Graphics::new();
    square.begin_fill(0xFFFFFF);
    square.draw_rect(0.0, 0.0, 50, 50);
    pixi.add_child(&square);

    body.append_child(&pixi.view());

    square.set_x(300.0);
    square.set_y(300.0);

    square.set_width(100.0);
    
    square.set_angle(45.0);
}
