use crate::wasm4::*;

const HEART_WIDTH: u32 = 9;
const HEART_HEIGHT: u32 = 9;
const HEART_FLAGS: u32 = 0; // BLIT_1BPP
const HEART: [u8; 11] = [ 0x9c,0x84,0x00,0x00,0x00,0x00,0x02,0x03,0x83,0xe3,0x80 ];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Heart {
    pub body: Point,
    pub hp: u8,        // HP atual (0-60)
    pub max_hp: u8,    // HP máximo (60)
    damage_cooldown: u8, // Cooldown para evitar dano múltiplo
}

impl Heart {
    pub fn new() -> Self {
        Self {
            body: Point {
                x: 20 + (120 - 9) / 2, // centralizado no eixo X
                y: 70 + (70 - 9) / 2,  // centralizado no eixo Y
            },
            hp: 60,           // HP inicial
            max_hp: 60,       // HP máximo
            damage_cooldown: 0,
        }
    }
   
    // Função para resetar o heart (para recomeçar jogo)
    pub fn reset(&mut self) {
        self.body = Point {
            x: 20 + (120 - 9) / 2,
            y: 70 + (70 - 9) / 2,
        };
        self.hp = 60;
        self.damage_cooldown = 0;
    }
   
    pub fn update(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };
       
        // Atualiza cooldown de dano
        if self.damage_cooldown > 0 {
            self.damage_cooldown -= 1;
        }
       
        if gamepad & BUTTON_LEFT != 0 {
            self.body.x -= 1;
        }
        if gamepad & BUTTON_RIGHT != 0 {
            self.body.x += 1;
        }
        if gamepad & BUTTON_UP != 0 {
            self.body.y -= 1;
        }
        if gamepad & BUTTON_DOWN != 0 {
            self.body.y += 1;
        }
       
        // Mantém o coração dentro da área jogável
        self.body.x = self.body.x.clamp(21, 130);
        self.body.y = self.body.y.clamp(71, 130);
    }
   
    // Função para receber dano
    pub fn take_damage(&mut self, damage: u8) -> bool {
        if self.damage_cooldown == 0 && self.hp > 0 {
            if self.hp > damage {
                self.hp -= damage;
            } else {
                self.hp = 0;
            }
            self.damage_cooldown = 10; // 10 frames de invencibilidade
            return true; // Dano foi aplicado
        }
        false // Sem dano (cooldown ativo)
    }
   
    // Verifica se está morto
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
   
    pub fn draw(&self) {
        // Se está no cooldown de dano, pisca
        if self.damage_cooldown > 0 && self.damage_cooldown % 4 < 2 {
            return; // Não desenha (efeito de piscar)
        }
       
        unsafe { *DRAW_COLORS = 0x14 }
        blit(&HEART, self.body.x, self.body.y, HEART_WIDTH, HEART_HEIGHT, HEART_FLAGS);
    }
   
    pub fn draw_hp_bar(&self) {
        let bar_x: i32 = 10;
        let bar_y: i32 = 145;
        let bar_width: u32 = 140;
        let bar_height: u32 = 12; // Aumentei a altura para caber o texto
       
        // Borda da barra
        unsafe { *DRAW_COLORS = 0x21 }
        rect(bar_x, bar_y, bar_width, bar_height);
       
        // Fundo da barra (vazio)
        unsafe { *DRAW_COLORS = 0x11 }
        rect(bar_x + 1, bar_y + 1, bar_width - 2, bar_height - 2);
       
        // Barra de HP (preenchida)
        if self.hp > 0 {
            let hp_width = ((bar_width - 2) as f32 * (self.hp as f32 / self.max_hp as f32)) as u32;
           
            // Cor da barra baseada no HP
            let color = if self.hp < 21 {
                0x41 // Vermelho quando HP baixo
            } else {
                0x31 // Verde quando HP alto
            };
            unsafe { *DRAW_COLORS = color }
            rect(bar_x + 1, bar_y + 1, hp_width.max(1), bar_height - 2);
        }
       
        // Texto do HP DENTRO da barra
        unsafe { *DRAW_COLORS = 0x12 } // Texto branco
        let hp_text = format!("HP: {}/{}", self.hp, self.max_hp);
        
        // Centraliza o texto verticalmente dentro da barra
        let text_y = bar_y + 2; // 2 pixels de padding do topo da barra
        text(&hp_text, bar_x + 4, text_y); // 4 pixels de padding da esquerda
    }
}