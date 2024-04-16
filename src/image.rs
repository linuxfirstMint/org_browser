use ft::{face::LoadFlag, ffi::FT_Int, Bitmap, GlyphSlot, Matrix, Vector};

use crate::{
    frame::{BUFFER_HEIGHT, BUFFER_WIDTH},
    render::{ft_init, Rendering},
};

pub struct Image {
    pub size: Vec<Vec<u8>>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            size: vec![vec![0; BUFFER_HEIGHT as usize]; BUFFER_WIDTH as usize],
        }
    }

    pub fn font(&self) {
        todo!()
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        pen: &mut Vector,
        target_height: i32,
        matrix: &mut Matrix,
        slot: GlyphSlot,
    ) {
        let face = ft_init();

        for c in text.chars() {
            face.set_transform(matrix, pen);
            face.load_char(c as usize, LoadFlag::RENDER).unwrap();
        }

        self.draw_bitmap(
            slot.bitmap(),
            slot.bitmap_left(),
            target_height - slot.bitmap_top(),
        );

        pen.x += slot.advance().x;
        pen.y += slot.advance().y;
    }

    fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= BUFFER_WIDTH.into() || y >= BUFFER_HEIGHT.into()
    }

    pub fn draw_bitmap(&mut self, bitmap: Bitmap, x: FT_Int, y: FT_Int) {
        let x_max = x + bitmap.width();
        let y_max = y + bitmap.rows();

        for i in x..x_max {
            for j in y..y_max {
                if self.is_out_of_bounds(i, j) {
                    continue;
                }

                let p = (i - x) as usize;
                let q = (j - x) as usize;

                self.size[j as usize][i as usize] |=
                    bitmap.buffer()[q * bitmap.width() as usize + p];
            }
        }
    }
}

impl Rendering for Image {
    fn render(&self) {
        self.font()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod image {
        use ft::bitmap;

        use super::*;

        #[test]
        fn vector_initialization() {
            let image = Image::new();

            for i in 0..BUFFER_WIDTH {
                for j in 0..BUFFER_HEIGHT {
                    assert_eq!(image.size[i as usize][j as usize], 0);
                }
            }
        }

        #[test]
        fn render_font() {
            let image = Image::new();
            image.font()
        }
    }
}
