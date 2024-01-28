use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];


/*
 Os traits "Copy" e "Clone" permitem que o tipo "Direction" seja copiado e clonado, 
 enquanto o trait "PartialEq" permite que sejam feitas comparações de igualdade entre valores do tipo "Direction".
*/
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
   Up,
   Down,
   Left,
   Right, 
}

impl Direction {
   //verifica a posicao oposta a que a snake esta indo
   pub fn oppositive(&self)  -> Direction {
      match *self {
         Direction::Up => Direction::Down,    
         Direction::Down => Direction::Up,    
         Direction::Left => Direction::Right,    
         Direction::Right => Direction::Left,    
      }
   }
}

struct Block {
   x: i32,
   y: i32,
}

pub struct Snake {
   direction: Direction,
   body: LinkedList<Block>,
   tail: Option<Block>,
}