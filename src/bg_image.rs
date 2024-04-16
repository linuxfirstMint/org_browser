use crate::{
    frame::{BUFFER_HEIGHT, BUFFER_WIDTH},
    render::Rendering,
};

pub struct BGimage {
    pub size: Vec<Vec<u16>>,
    // pub size: [[u16; BUFFER_HEIGHT as usize]; BUFFER_WIDTH as usize],
}

impl BGimage {
    pub fn new() -> Self {
        BGimage {
            // size: [[0; BUFFER_HEIGHT as usize]; BUFFER_WIDTH as usize],
            size: vec![vec![0; BUFFER_HEIGHT as usize]; BUFFER_WIDTH as usize],
        }
    }

    pub fn title_bar(&self) {
        todo!()
    }

    pub fn back_groud(&self) {
        todo!();
    }
}

impl Rendering for BGimage {
    fn render(&self) {
        self.title_bar();
        self.back_groud()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_initialization() {
        let bgimage = BGimage::new();

        for i in 0..BUFFER_WIDTH {
            for j in 0..BUFFER_HEIGHT {
                assert_eq!(bgimage.size[i as usize][j as usize], 0);
            }
        }
    }

    #[test]
    fn render_title_bar() {
        let bgimage = BGimage::new();
        bgimage.title_bar()
    }

    #[test]
    fn render_back_groud() {
        let bgimage = BGimage::new();
        bgimage.back_groud()
    }
}
