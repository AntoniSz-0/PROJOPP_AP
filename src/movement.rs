use crate::{BOARD_HEIGHT, BOARD_WIDTH};

pub fn move_snake_head(px: u32, py: u32, direction: u32) -> Result<(u32, u32), i32> {
    match direction {
        0 => {
            if py > 0 {
                Ok((px, py - 1)) // up
            } else {
                Err(0)
            }
        },
        1 => {
            if px < BOARD_WIDTH-1 {
                Ok((px + 1, py)) // right
            } else {
                Err(0)
            }
        },
        2 => {
            if py < BOARD_HEIGHT-3 {
                Ok((px, py + 1)) // down
            } else {
                Err(0)
            }
        },
        3 => {
            if px > 0 {
                Ok((px - 1, py)) // left
            } else {
                Err(0)
            }
        },
        _ => Ok((px, py))
    }
}

pub fn move_snake_body(snake_body: &mut Vec<(u32, u32, u32, u32)>, new_head_pos_x: u32, new_head_pos_y: u32) {
    snake_body[0].2 = snake_body[0].0;
    snake_body[0].3 = snake_body[0].1;
    snake_body[0].0 = new_head_pos_x;
    snake_body[0].1 = new_head_pos_y;
    for i in 1..snake_body.len() {
        snake_body[i].2 = snake_body[i].0;
        snake_body[i].3 = snake_body[i].1;
        snake_body[i].0 = snake_body[i-1].2;
        snake_body[i].1 = snake_body[i-1].3;
    }
}