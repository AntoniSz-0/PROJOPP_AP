use std::io;
use std::sync::{Arc, Mutex};
use crate::{BOARD_HEIGHT, BOARD_WIDTH, any_snake_segment_here, SnakeDirection};
use crate::snake_utils::rng_on_board;

pub fn draw(positions: &Vec<(u32, u32, u32, u32)>, apple_pos: (u32, u32)) -> i32 {
    print!("{esc}c", esc = 27 as char);


    //top border
    for _ in 0..BOARD_WIDTH+2 {
        print!("_");
    }
    println!();

    for i in 0..BOARD_HEIGHT-2 {
        print!("|"); // left border
        for j in 0..BOARD_WIDTH {
            if positions[0].0 == j && positions[0].1 == i {
                print!("0");
            } else if any_snake_segment_here(positions, j, i) {
                print!("~")
            } else if apple_pos == (j, i) {
                print!("+")
            } else {
                print!(" ")
            }
        }
        println!("|"); //right border
    }

    //bottom border
    for _ in 0..BOARD_WIDTH+2 {
        print!("\"");
    }
    println!();
    0
}

pub fn starting_info() -> i32 {
    println!("\n\n\n\n\n\n\n\n\n                     To set the snake’s direction, use the arrow keys on your keyboard.");
    println!("                          To enlarge the snake, eat the food represented by '+'. \n\n\n\n\n\n\n\n\n\n");

    println!("                                             Press Enter to play");
    let mut wait= Default::default();
    io::stdin()
        .read_line(&mut wait)
        .expect("Something went wrong");

    0
}

pub fn handle_defeat(snake_body: &mut Vec<(u32, u32, u32, u32)>, direction: Arc<Mutex<SnakeDirection>>, apple_x: &mut u32, apple_y: &mut u32) -> bool {
    for _ in 5..(BOARD_WIDTH / 2) {
        print!(" ")
    }
    println!("YOU LOST! :(");

    if ask_to_play_again() {
        (*snake_body).clear();
        let head_pos_x: u32 = BOARD_WIDTH / 2;
        let head_pos_y: u32 = BOARD_HEIGHT / 2 - 1;
        snake_body.push((head_pos_x, head_pos_y, head_pos_x, head_pos_y));
        let mut dir = direction.lock().unwrap(); // Lock the mutex to get access to the data
        dir.current = 4;
        let (new_apple_x, new_apple_y) = rng_on_board(snake_body);
        *apple_x = new_apple_x;
        *apple_y = new_apple_y;
        false
    } else {
        true
    }
}

fn ask_to_play_again() -> bool {
    let mut question_answer = String::new();
    loop {
        println!("Would you like to play again?");

        io::stdin()
            .read_line(&mut question_answer)
            .expect("Failed to read line");

        question_answer = question_answer.trim().to_lowercase();

        if question_answer == "yes" || question_answer == "y" {
            return true;
        } else if question_answer == "no" || question_answer == "n" {
            return false;
        } else {
            println!("Please enter \"yes\" or \"no\"");
            question_answer = "".parse().unwrap();
            continue;
        }
    }
}