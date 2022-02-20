use crate::NUM_COLS;

pub type ScoreBar = Vec<String>;

pub fn new_score_bar() -> ScoreBar {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        cols.push(' '.to_string());
    }
    cols
}

