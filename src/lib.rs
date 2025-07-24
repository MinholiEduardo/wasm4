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
use enemy::{SANS3, SANS3_FLAGS, SANS3_HEIGHT, SANS3_WIDTH};
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
                let x_pos = 160 - SANS3_WIDTH as i32 + 10; // Ajuste para ficar bonito
                let y_pos = 160 - SANS3_HEIGHT as i32 + 10;
                blit(&SANS3, x_pos, y_pos, SANS3_WIDTH, SANS3_HEIGHT, SANS3_FLAGS);

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
                text("PRESS X TO", 40, 80);
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
                // Tela de vitória CENTRALIZADA e com instruções
                *DRAW_COLORS = 0x31;
                
                // Título centralizado
                text("CONGRATULATIONS!", 15, 30);
                
                // Mensagem principal centralizada
                text("YOU SURVIVED", 35, 50);
                text("85 SECONDS!", 38, 65);
                
                // Resultado centralizado
                text("YOU WIN!", 45, 85);
                
                // Instruções centralizadas
                text("PRESS X TO", 40, 110);
                text("PLAY AGAIN", 40, 125);

                let gamepad = *GAMEPAD1;
                if gamepad & BUTTON_2 != 0 { // BUTTON_2 é o X
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