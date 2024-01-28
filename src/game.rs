use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_rectangle, draw_block};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
   snake: Snake,
   food_exist: bool,
   food_x: i32,
   food_y: i32,
   width: i32,
   heigh: i32,
   game_over: bool,
   waiting_time: f64,
}

impl Game {
   pub fn new(width: i32, heigh: i32) -> Game {
      Game {
         snake: Snake::new(2, 2),
         waiting_time: 0.0,
         food_exist: true,
         food_x: 4,
         food_y: 6,
         width,
         heigh,
         game_over: false
      }
   }
}