/*
"rectangle" é usada para desenhar um retângulo na janela. 
O tipo "Color" é usado para especificar a cor de um elemento gráfico. 
O contexto "Context" é usado para armazenar informações sobre o ambiente de desenho atual, 
enquanto o contexto "G2d" é usado para armazenar informações sobre o estado gráfico atual.
*/

use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;


/*
A constante "BLOCK_SIZE" é usada como um fator de conversão. 
Ela é definida como "25.0", o que significa que cada "bloco" no jogo corresponde a 25 pixels na janela.
*/
const BLOCK_SIZE: f64 = 25.0;


/*
A função "to_coord" recebe um parâmetro "game_coord" do tipo "i32" e retorna um valor do tipo "f64". 
Ela é usada para converter coordenadas do jogo (que são inteiros) para coordenadas da janela (que são do tipo "f64").
*/
pub fn to_cood(game_coord: i32) -> f64 {
   (game_coord as f64) * BLOCK_SIZE
}


/*
A função "draw_block" é usada para desenhar um "bloco" na janela. 
Ela recebe a cor do bloco (que é um valor do tipo "Color"), as coordenadas "x" e "y" do jogo 
e os contextos "con" e "g" como argumentos. Em seguida, ela converte as coordenadas do jogo para 
coordenadas da janela usando a função "to_coord" e usa a função "rectangle" da biblioteca "piston_window" 
para desenhar um quadrado na janela. O quadrado terá 25 pixels de largura e altura e será desenhado 
na posição (x, y) da janela, usando a cor especificada.
*/
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
   let gui_x = to_cood(x);
   let gui_y = to_cood(y);

   rectangle(
      color, 
      [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
      con.transform, 
      g,
   );
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
   let x = to_cood(x);
   let y = to_cood(y);

   rectangle(
      color,
      [x, y, BLOCK_SIZE*(width as f64), BLOCK_SIZE*(height as f64)],
      con.transform,
      g,
   );
}