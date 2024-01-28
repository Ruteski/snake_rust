/*
- extern crate rand: Essa linha é utilizada para importar a biblioteca rand, que será utilizada para gerar números aleatórios no jogo.

- extern crate piston_window: Essa linha é utilizada para importar a biblioteca piston_window, que é uma das bibliotecas mais utilizadas para desenvolver jogos em Rust.

- mod draw;: Essa linha está declarando que existe um módulo (um conjunto de códigos organizados em uma estrutura de arquivos) chamado "draw" e que ele será utilizado neste código.

- mod snake;: Essa linha está declarando que existe um módulo chamado "snake" e que ele será utilizado neste código.

- mod game;: Essa linha está declarando que existe um módulo chamado "game" e que ele será utilizado neste código.

- use piston_window::*;: Essa linha está importando todas as definições do módulo "piston_window".

- use piston_window::types::Color;: Essa linha está importando a definição de "Color" do módulo "piston_window::types".

- use game::Game;: Essa linha está importando a definição "Game" do módulo "game".

- use draw::to_coord_u32;: Essa linha está importando a definição "to_coord_u32" do módulo "draw".
*/

extern crate rand; // importa lib instalada no cargo
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_cood_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];


fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake Rust", 
        [to_cood_u32(width), to_cood_u32(height)]
    ).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g|{
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

 
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
