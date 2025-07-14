#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod heart;
mod enemy;
mod projectile;

use heart::Heart;
use wasm4::*;
use enemy::Enemy;

static mut HEART: Option<Heart> = None;
static mut ENEMY: Option<Enemy> = None;

#[no_mangle]
fn update() {

    // Inicializa o coração apenas uma vez
    unsafe {
        if HEART.is_none() {
            HEART = Some(Heart::new());
        }
        if ENEMY.is_none() {
            ENEMY = Some(Enemy::new());
        }        
    }    

    // Configura paleta
    unsafe {
        *PALETTE = [
            0x000000, // Preto
            0x7f7f7f, // Cinza
            0xffffff, // Branco
            0xff0000, // Vermelho
        ];
    }

    // Fundo do retângulo (campo de movimento)
    unsafe { *DRAW_COLORS = 0x31 }
    rect(20, 70, 120, 70);

    unsafe {
        if let (Some(ref mut heart), Some(ref mut enemy)) = (HEART.as_mut(), ENEMY.as_mut()) {
            if heart.life > 0 {
                heart.update();
                enemy.update(heart);
            }

            heart.draw();
            heart.draw_life_bar();
            enemy.draw();

            // Fim de jogo
            if heart.life == 0 {
                *DRAW_COLORS = 0x21;
                text("FIM DE JOGO", 36, 80);
            }
        }
    }
}
