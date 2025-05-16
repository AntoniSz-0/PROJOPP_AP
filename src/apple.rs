use crate::snake_utils::{add_snake_segment, rng_on_board};


pub fn generate_apple(snake_body: &mut Vec<(u32, u32, u32, u32)>, apple_x: &mut u32, apple_y: &mut u32) {
    if (snake_body[0].0, snake_body[0].1) == (*apple_x, *apple_y) {
        add_snake_segment(snake_body);
        let (new_apple_x, new_apple_y) = rng_on_board(snake_body);
        *apple_x = new_apple_x;
        *apple_y = new_apple_y;
    }
}