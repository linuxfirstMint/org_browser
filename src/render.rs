use ft::face::LoadFlag;
use ft::{Face, Library};

use crate::font::FONT_PATH;

pub fn ft_init() -> Face {
    let lib = Library::init().unwrap();
    // Load a font face
    let face = lib.new_face(FONT_PATH, 0).unwrap();

    face
}

pub trait Rendering {
    fn render(&self);
}
