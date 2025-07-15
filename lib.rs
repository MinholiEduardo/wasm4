#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod heart;
mod enemy;
mod projectile;

use heart::Heart;
use wasm4::*;
use enemy::Enemy;
use enemy::{SANS, SANS_FLAGS, SANS_HEIGHT, SANS_WIDTH};

// Enum para controlar o estado do jogo
enum GameState {
    StartScreen,
    Playing,
    GameOver,
}

static mut HEART: Option<Heart> = None;
static mut ENEMY: Option<Enemy> = None;
static mut GAME_STATE: GameState = GameState::StartScreen; // Estado inicial

#[no_mangle]
fn update() {
    unsafe {
        match GAME_STATE {
            GameState::StartScreen => {
                // Lógica da Tela Inicial
                *PALETTE = [0x000000, 0xffffff, 0xffffff, 0x000000]; // Paleta simples para a tela inicial
                *DRAW_COLORS = 0x12;
                text("Comic-Sans", 10, 10);
                text("Aperte Z", 10, 30 );
                text("Para iniciar", 10, 40);

                *DRAW_COLORS = 0x02; // Cor 3 para o sprite, fundo transparente
                let x_pos = 160 - SANS_WIDTH as i32 + 10; // Ajuste para ficar bonito
                let y_pos = 160 - SANS_HEIGHT as i32 + 10;
                blit(&SANS, x_pos, y_pos, SANS_WIDTH, SANS_HEIGHT, SANS_FLAGS);

                // Verifica se qualquer botão foi pressionado para iniciar o jogo
                let gamepad = *GAMEPAD1;
                if gamepad != 0 {
                    GAME_STATE = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Lógica do Jogo Principal (o que já existia)
                if HEART.is_none() {
                    HEART = Some(Heart::new());
                }
                if ENEMY.is_none() {
                    ENEMY = Some(Enemy::new());
                }

                *PALETTE = [
                    0x000000, // Preto
                    0x7f7f7f, // Cinza
                    0xffffff, // Branco
                    0xff0000, // Vermelho
                ];

                *DRAW_COLORS = 0x31;
                rect(20, 70, 120, 70);

                if let (Some(ref mut heart), Some(ref mut enemy)) = (HEART.as_mut(), ENEMY.as_mut()) {
                    if heart.life > 0 {
                        heart.update();
                        enemy.update(heart);
                    }

                    heart.draw();
                    heart.draw_life_bar();
                    enemy.draw();

                    if heart.life == 0 {
                        GAME_STATE = GameState::GameOver;
                    }
                }
            }
            GameState::GameOver => {
                // Lógica de Fim de Jogo
                *DRAW_COLORS = 0x21;
                text("FIM DE JOGO", 36, 80);
            }
        }
    }
}