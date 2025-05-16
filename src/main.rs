mod movement;
mod apple;
mod ui;
mod snake_utils;

use movement::move_snake_head;
use apple::generate_apple;
use ui::{draw, starting_info, handle_defeat};
use snake_utils::{rng_on_board, any_snake_segment_here, is_opposite};

use crossterm::event::{self, Event, KeyCode};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::process::Command;
use crate::movement::move_snake_body;

static BOARD_WIDTH: u32 = 40;
static BOARD_HEIGHT: u32 = 20;

struct SnakeDirection {
    current: u32,
    pending: Option<u32>,
}

fn main() {
    env_logger::init();
    /// ###### Vector storing current and previous snake coordinates
    /// ## Arguments
    /// * 1st u32 - current x-coordinate
    /// * 2nd u32 - current y-coordinate
    /// * 3rd u32 - previous x-coordinate
    /// * 4th u32 - previous y-coordinate
    let mut snake_body: Vec<(u32, u32, u32, u32)> = Vec::new();

    {
        let head_pos_x: u32 = BOARD_WIDTH / 2;
        let head_pos_y: u32 = BOARD_HEIGHT / 2 - 1;
        snake_body.push((head_pos_x, head_pos_y, head_pos_x, head_pos_y));
    }

    // creating a variable shared between the input thread and the main loop
    let direction = Arc::new(Mutex::new(SnakeDirection { current: 4, pending: None }));

    // clearing the terminal
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();

    starting_info();

    // creating a reference to 'direction'
    let direction_clone = Arc::clone(&direction);
    // thread for changing the direction
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    let new_dir = match key_event.code {
                        KeyCode::Up => 0,
                        KeyCode::Right => 1,
                        KeyCode::Down => 2,
                        KeyCode::Left => 3,
                        _ => continue,
                    };
                    let mut dir = direction_clone.lock().unwrap();
                    dir.pending = Some(new_dir);
                }
            }
        }
    });
    let (mut apple_x, mut apple_y) = rng_on_board(&snake_body);
    'main: loop {
        {
            {
                let mut dir = direction.lock().unwrap();
                if let Some(pending) = dir.pending {
                    if !is_opposite(dir.current, pending) {
                        dir.current = pending;
                    }
                    dir.pending = None;
                }
            }
            let current_dir = direction.lock().unwrap().current;
            match move_snake_head(snake_body[0].0, snake_body[0].1, current_dir) {
                Ok((new_head_pos_x, new_head_pos_y)) => {
                    move_snake_body(&mut snake_body, new_head_pos_x, new_head_pos_y)
                },
                Err(_) => {
                    if handle_defeat(&mut snake_body, direction.clone(), &mut apple_x, &mut apple_y) {
                        break
                    }
                }
            }
        }

        generate_apple(&mut snake_body, &mut apple_x, &mut apple_y);

        draw(&snake_body, (apple_x, apple_y));
        {
            if snake_body.len() > 1 {
                let snake_body_c = snake_body.clone();
                let head_coords = (snake_body_c[0].0, snake_body_c[0].1);
                let mut segment_coords: (u32, u32);
                for i in 1..snake_body_c.len() {
                    segment_coords = (snake_body_c[i].0, snake_body_c[i].1);
                    if head_coords == segment_coords {
                        if handle_defeat(&mut snake_body, direction.clone(), &mut apple_x, &mut apple_y) {
                            break 'main
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
}