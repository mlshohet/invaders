use crate::frame::{Drawable, Frame};

pub struct Score {
    x: usize,
}

impl Score {
    pub fn new() -> Self {
        Self { 
            x: 0,
        }
    }

    pub fn update_score(&mut self) {
        self.x += 10;

    }
}

impl Drawable for Score {
    fn draw (&self, frame: &mut Frame) {
        frame[0][0] = format!("SCORE: {}", self.x);
    }
}
