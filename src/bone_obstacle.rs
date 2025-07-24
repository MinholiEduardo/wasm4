// src/bone_obstacle.rs

use crate::wasm4::*;
use crate::heart::{Heart, Point};

// OSSO PP
pub const P_P_WIDTH: u32 = 5;
pub const P_P_HEIGHT: u32 = 9;
pub const P_P_FLAGS: u32 = 0; // BLIT_1BPP
pub const P_P: [u8; 6] = [ 0x20,0x23,0x18,0xc6,0x20,0x20 ];

// OSSO P
pub const P_WIDTH: u32 = 5;
pub const P_HEIGHT: u32 = 18;
pub const P_FLAGS: u32 = 0; // BLIT_1BPP
pub const P: [u8; 12] = [ 0x20,0x23,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc6,0x31,0x01,0x00 ];

// OSSO M
pub const M_WIDTH: u32 = 5;
pub const M_HEIGHT: u32 = 24;
pub const M_FLAGS: u32 = 0; // BLIT_1BPP
pub const M: [u8; 15] = [ 0x20,0x23,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc4,0x04 ];

// OSSO G
pub const G_WIDTH: u32 = 5;
pub const G_HEIGHT: u32 = 36;
pub const G_FLAGS: u32 = 0; // BLIT_1BPP
pub const G: [u8; 23] = [ 0x20,0x23,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc6,0x31,0x8c,0x63,0x18,0xc6,0x31,0x8c,0x40,0x40 ];

// Enum para facilitar a seleção do tipo de osso
#[derive(Clone, Copy)]
pub enum BoneType {
    Pp,
    P,
    M,
    G,
}

impl BoneType {
    // Retorna o sprite e suas dimensões
    pub fn get_sprite_data(&self) -> (&'static [u8], u32, u32) {
        match self {
            BoneType::Pp => (&P_P, P_P_WIDTH, P_P_HEIGHT),
            BoneType::P => (&P, P_WIDTH, P_HEIGHT),
            BoneType::M => (&M, M_WIDTH, M_HEIGHT),
            BoneType::G => (&G, G_WIDTH, G_HEIGHT),
        }
    }
}

pub struct BoneObstacle {
    pos: Point,
    velocity_x: i32,
    bone_type: BoneType,
}

impl BoneObstacle {
    pub fn new(y: i32, from_left: bool, bone_type: BoneType, speed: i32) -> Self {
        let (_, width, _) = bone_type.get_sprite_data();
        let x = if from_left {
            20 - width as i32 // Começa fora da tela, à esquerda
        } else {
            140 // Começa na borda direita da área de jogo
        };

        let velocity_x = if from_left { speed } else { -speed };

        Self {
            pos: Point { x, y },
            velocity_x,
            bone_type,
        }
    }

    pub fn update(&mut self) {
        self.pos.x += self.velocity_x;
    }

    pub fn draw(&self) {
        let (sprite, width, height) = self.bone_type.get_sprite_data();
        unsafe { *DRAW_COLORS = 0x13 }
        blit(sprite, self.pos.x, self.pos.y, width, height, P_P_FLAGS);
    }

    // Verifica se o obstáculo já saiu completamente da tela
    pub fn is_off_screen(&self) -> bool {
        let (_, width, _) = self.bone_type.get_sprite_data();
        if self.velocity_x > 0 { // Movendo para a direita
            self.pos.x > 140
        } else { // Movendo para a esquerda
            self.pos.x + (width as i32) < 20
        }
    }

    // Verifica colisão com o coração
    pub fn collides_with_heart(&self, heart: &Heart) -> bool {
        let (_, bone_width, bone_height) = self.bone_type.get_sprite_data();
        
        let heart_left = heart.body.x;
        let heart_right = heart.body.x + 9; // HEART_WIDTH
        let heart_top = heart.body.y;
        let heart_bottom = heart.body.y + 9; // HEART_HEIGHT

        let bone_left = self.pos.x;
        let bone_right = self.pos.x + bone_width as i32;
        let bone_top = self.pos.y;
        let bone_bottom = self.pos.y + bone_height as i32;

        // Verificação de sobreposição AABB (Axis-Aligned Bounding Box)
        heart_left < bone_right &&
        heart_right > bone_left &&
        heart_top < bone_bottom &&
        heart_bottom > bone_top
    }
}