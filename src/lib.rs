#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod heart;
mod enemy;
mod projectile;
mod music;

use heart::Heart;
use wasm4::*;
use enemy::Enemy;
use enemy::{SANS, SANS_FLAGS, SANS_HEIGHT, SANS_WIDTH};
use music::update_music;

// Enum para controlar o estado do jogo
enum GameState {
    StartScreen,
    Playing,
    GameOver,
    Victory,
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
                text("PRESS Z", 10, 30 );
                text("TO START", 10, 40);

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
                // Lógica do Jogo Principal 
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
                    if !heart.is_dead() {
                        heart.update();
                        enemy.update(heart);
                        
                        // Verifica se o jogador venceu
                        if enemy.has_player_won() {
                            GAME_STATE = GameState::Victory;
                        }
                    }
                    
                    heart.draw();
                    heart.draw_hp_bar(); // Muda de draw_life_bar para draw_hp_bar
                    enemy.draw();
                    
                    if heart.is_dead() {
                        GAME_STATE = GameState::GameOver;
                    }
                }

                update_music();  

            }      
            GameState::GameOver => {
                // Tela de Game Over
                *DRAW_COLORS = 0x21;
                text("GAME OVER", 45, 60);
                text("PRESS Z TO", 40, 80);
                text("RESTART", 50, 100);
                
                // Verifica se Z foi pressionado para recomeçar
                let gamepad = *GAMEPAD1;
                if gamepad & BUTTON_1 != 0 { // BUTTON_1 é o Z
                    // Reset do jogo
                    if let (Some(ref mut heart), Some(ref mut enemy)) = (HEART.as_mut(), ENEMY.as_mut()) {
                        heart.reset();
                        enemy.reset();
                    }
                    GAME_STATE = GameState::Playing;
                }
            }

            GameState::Victory => {
            // Tela de vitória
                *DRAW_COLORS = 0x31;
                text("CONGRATULATIONS!", 20, 60);
                text("YOU SURVIVED!", 30, 80);
                text("90 SECONDS!", 35, 100);
                text("YOU WIN!", 40, 120);

                let gamepad = *GAMEPAD1;
                if gamepad & BUTTON_1 != 0 { // BUTTON_1 é o Z
                    // Reset do jogo
                    if let (Some(ref mut heart), Some(ref mut enemy)) = (HEART.as_mut(), ENEMY.as_mut()) {
                        heart.reset();
                        enemy.reset();
                    }
                    GAME_STATE = GameState::Playing;
                }
            }
        }
    }
}