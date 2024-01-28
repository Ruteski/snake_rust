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