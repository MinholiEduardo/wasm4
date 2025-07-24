#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod heart;
mod enemy;
mod projectile;
mod music;
mod bone_obstacle;

use heart::Heart;
use wasm4::*;
use enemy::Enemy;
use enemy::{SANS3, SANS3_FLAGS, SANS3_HEIGHT, SANS3_WIDTH};
use music::update_music;

// Enum para controlar o estado do jogo
enum GameState {
    StartScreen,
    FadeToQuote,
    QuoteScreen1, 
    QuoteScreen2, 
    FadeToGame,
    Playing,
    GameOver,
    Victory,
}

static mut HEART: Option<Heart> = None;
static mut ENEMY: Option<Enemy> = None;
static mut GAME_STATE: GameState = GameState::StartScreen;
static mut QUOTE_TIMER: u32 = 0;
static mut TRANSITION_TIMER: u32 = 0;

#[no_mangle]
fn update() {
    unsafe {
        match GAME_STATE {
            GameState::StartScreen => {
                *PALETTE = [0x000000, 0xffffff, 0xffffff, 0x000000];
                *DRAW_COLORS = 0x12;
                text("Comic-Sans", 10, 10);
                text("PRESS X", 10, 30 );
                text("TO START", 10, 40);

                *DRAW_COLORS = 0x02;
                let x_pos = 160 - SANS3_WIDTH as i32 + 10;
                let y_pos = 160 - SANS3_HEIGHT as i32 + 10;
                blit(&SANS3, x_pos, y_pos, SANS3_WIDTH, SANS3_HEIGHT, SANS3_FLAGS);

                let gamepad = *GAMEPAD1;
                if gamepad & BUTTON_1 != 0 {
                    GAME_STATE = GameState::FadeToQuote;
                    TRANSITION_TIMER = 0;
                }
            }

            GameState::FadeToQuote => {
                *PALETTE = [0; 4];
                TRANSITION_TIMER += 1;
                if TRANSITION_TIMER > 20 {
                    GAME_STATE = GameState::QuoteScreen1;
                    QUOTE_TIMER = 0;
                }
            }

            GameState::QuoteScreen1 => {
                *PALETTE = [0x000000, 0xffffff, 0, 0];
                *DRAW_COLORS = 0x12;

                text("it's a beautiful", 5, 40);
                text("day outside.", 5, 50);

                text("birds are singing,", 5, 70); // Espaço extra de 20px no Y
                text("flowers are", 5, 80);
                text("blooming...", 5, 90);


                text("on days like these,", 5, 110); // Espaço extra de 20px no Y
                text("kids like you...", 5, 120);

                QUOTE_TIMER += 1;
                if QUOTE_TIMER > 300 { // 5 segundos
                    // Transição para a segunda tela de citação
                    GAME_STATE = GameState::QuoteScreen2;
                    QUOTE_TIMER = 0; // Reseta o timer para a próxima tela
                }
            }
            
            GameState::QuoteScreen2 => {
                // Define uma paleta que inclui vermelho para o texto
                *PALETTE = [0x000000, 0x000000, 0x000000, 0xff0000]; // Cor 4 é vermelho
                *DRAW_COLORS = 0x14; // Fundo preto (cor 1), texto vermelho (cor 4)

                // Centralizando o texto
                text("Should be burning", 10, 70);
                text("in hell.", 42, 82);
                
                QUOTE_TIMER += 1;
                if QUOTE_TIMER > 150 { // Duração menor, para mais impacto (2.5 segundos)
                    GAME_STATE = GameState::FadeToGame;
                    TRANSITION_TIMER = 0;
                }
            }

            GameState::FadeToGame => {
                *PALETTE = [0; 4];
                TRANSITION_TIMER += 1;
                if TRANSITION_TIMER > 20 {
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

                text("THANKS FOR PLAYING", 10, 70);

                
                // Instruções centralizadas
                text("PRESS Z TO", 40, 110);
                text("PLAY AGAIN", 40, 125);

                let gamepad = *GAMEPAD1;
                if gamepad & BUTTON_2 != 0 { // BUTTON_2 é o X
                    // Reset do jogo
                    if let (Some(ref mut heart), Some(ref mut enemy)) = (HEART.as_mut(), ENEMY.as_mut()) {
                        heart.reset();
                        enemy.reset();
                    }
                    GAME_STATE = GameState::StartScreen;
                }
            }
        }
    }
}