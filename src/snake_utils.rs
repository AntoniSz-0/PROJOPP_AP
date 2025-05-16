use rand::Rng;
use crate::{BOARD_HEIGHT, BOARD_WIDTH};

pub fn rng_on_board(snake_body: &Vec<(u32, u32, u32, u32)>) -> (u32, u32) {
    loop {
        let mut rng = rand::rng();
        let x: u32 = rng.random_range(1..BOARD_WIDTH - 1);
        let y: u32 = rng.random_range(1..BOARD_HEIGHT - 3);
        if !any_snake_segment_here(snake_body, x, y) {
            return (x, y);
        }
    }
}

/// ###### Checks if any segment of the snake is on `x` and `y`
/// ## Arguments
/// * `x` - the x-coordinate to compare with `snake_body[n]` x-coordinate
/// * `y` - the y-coordinate to compare with `snake_body[n]` y-coordinate
pub fn any_snake_segment_here(snake_body: &Vec<(u32, u32, u32, u32)>, x: u32, y: u32) -> bool { // (except the head)
    for i in 1..snake_body.len() {
        if snake_body[i].0 == x && snake_body[i].1 == y{
            return true;
        }
    }
    false
}

pub fn is_opposite(current: u32, new: u32) -> bool {
    matches!((current, new), (0, 2) | (2, 0) | (1, 3) | (3, 1))
}

pub fn add_snake_segment(snake_body: &mut Vec<(u32, u32, u32, u32)>) -> i32 {
    snake_body.push((snake_body[0].2, snake_body[0].3, snake_body[0].2, snake_body[0].3));
    0
}