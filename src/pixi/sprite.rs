use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable, Sizable, Rotatable };

#[derive(Clone, PartialEq, Debug, Positionable, Sizable, Rotatable)]
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