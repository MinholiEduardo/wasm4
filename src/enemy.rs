use crate::wasm4::*;
use crate::heart::{Heart, Point};
use crate::projectile::{Projectile, ProjectileType};

// sans
pub const SANS_WIDTH: u32 = 50;
pub const SANS_HEIGHT: u32 = 50;
pub const SANS_FLAGS: u32 = 0; // BLIT_1BPP
pub const SANS: [u8; 313] = [
    0xff,0xff,0xf8,0x01,0xff,0xff,0xff,0xff,0xf0,0x00,0x0f,0xff,0xff,0xff,0xf8,0x00,0x03,0xff,
    0xff,0xff,0xfc,0x00,0x00,0x3f,0xff,0xff,0xff,0x00,0x00,0x0f,0xff,0xff,0xff,0x80,0x00,0x01,
    0xff,0xff,0xff,0xe1,0xe0,0x7c,0x7f,0xff,0xff,0xf8,0xfc,0x3f,0x1f,0xff,0xff,0xfe,0x73,0x0c,
    0xe7,0xff,0xff,0xff,0x9c,0xc3,0x39,0xff,0xff,0xff,0xf3,0xe0,0x7c,0xff,0xff,0xff,0xfe,0x09,
    0x90,0x7f,0xff,0xff,0xff,0x84,0xf2,0x0f,0xff,0xff,0xff,0xc4,0x3c,0x23,0xff,0xff,0xff,0xf1,
    0x80,0x1c,0xff,0xff,0xff,0xfc,0x7f,0xfa,0x3f,0xff,0xff,0xff,0x09,0x4b,0x1f,0xff,0xff,0xfe,
    0x61,0xd3,0x8d,0xff,0xff,0xff,0x0e,0x1f,0x82,0x3f,0xff,0xff,0xb0,0xe0,0x07,0x1b,0xff,0xff,
    0xff,0x1f,0xff,0xce,0x7f,0xff,0xff,0xe1,0x83,0x86,0xcf,0xff,0xfd,0xde,0x31,0x8f,0x7b,0xff,
    0xfe,0xf7,0xb7,0xc7,0xde,0x3f,0xff,0x7d,0xef,0x3d,0xf7,0xe7,0xff,0xdf,0x78,0xc4,0x79,0xf9,
    0xff,0xf7,0xc0,0xd1,0x50,0x7e,0x7f,0xf9,0xf7,0xd4,0x43,0xcf,0x9f,0xfe,0x7d,0xf1,0x14,0xfb,
    0xf7,0xff,0xcf,0x7d,0x44,0xbe,0xf9,0xff,0xf9,0xdf,0x51,0x2f,0xbc,0x7f,0xff,0x37,0xd7,0xc7,
    0xee,0x7f,0xff,0xec,0xf8,0x03,0xf2,0x3f,0xff,0xfe,0x00,0x3e,0x01,0x9f,0xff,0xff,0xff,0xff,
    0xff,0xff,0xff,0xff,0xfa,0x7f,0xf7,0xdf,0xff,0xff,0xfd,0x1f,0xfc,0xf7,0xff,0xff,0xff,0x4f,
    0xef,0x3d,0xff,0xff,0xff,0xd3,0xf5,0xdf,0x7f,0xff,0xff,0xf4,0xfd,0x73,0xef,0xff,0xff,0xf9,
    0x7f,0x5c,0xfb,0xff,0xff,0xfe,0xdf,0x97,0x3e,0xff,0xff,0xff,0xb7,0xed,0xef,0xbf,0xff,0xff,
    0xed,0xfb,0x7b,0x8f,0xff,0xff,0xfc,0x00,0xe0,0x17,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,
    0xff,0x88,0xff,0x83,0x1f,0xff,0xff,0xc1,0x7f,0xe1,0x83,0xff,0xff,0xe0,0x3f,0xff,0xc0,0x7f,
    0xff,0xf8,0x09,0xfc,0x20,0x3f,0xf0
];

pub struct Enemy {
    pub pos: Point,
    pub projectiles: Vec<Projectile>,
    attack_timer: u32,
    attack_pattern: u32,
    pattern_phase: u32, // Para subdivisões dentro de cada padrão
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 80, y: 20 },
            projectiles: Vec::new(),
            attack_timer: 0,
            attack_pattern: 0,
            pattern_phase: 0,
        }
    }

    pub fn update(&mut self, heart: &mut Heart) {
        self.attack_timer += 1;
        
        // Atualiza projéteis existentes
        self.projectiles.retain_mut(|projectile| {
            projectile.update();
            
            // Remove se saiu da tela
            if !projectile.is_on_screen() {
                return false;
            }
            
            // Verifica colisão com o heart usando o novo método
            if projectile.collides_with_heart(heart) {
                if heart.life > 0 {
                    heart.life -= 1;
                }
                return false; // Remove o projétil após hit
            }
            
            true
        });
        
        // Muda padrão de ataque a cada 240 frames (4 segundos a 60fps)
        if self.attack_timer % 240 == 0 {
            self.attack_pattern = (self.attack_pattern + 1) % 5;
            self.pattern_phase = 0; // Reset phase quando muda padrão
        }
        
        // Diferentes padrões de ataque
        match self.attack_pattern {
            0 => self.pattern_basic_shots(heart),
            1 => self.pattern_rectangle_barrage(heart),
            2 => self.pattern_mixed_sizes(heart),
            3 => self.pattern_corner_chaos(heart),
            4 => self.pattern_spiral_attack(heart),
            _ => {}
        }
    }
    
    // Padrão 0: Tiros básicos simples - mais fácil
    fn pattern_basic_shots(&mut self, heart: &Heart) {
        if self.attack_timer % 50 == 0 { // A cada ~0.83 segundos
            let spawn_point = Point { x: 80, y: 20 }; // Do topo
            let projectile = Projectile::new(
                spawn_point, 
                heart.body, 
                ProjectileType::Small
            );
            self.projectiles.push(projectile);
        }
    }
    
    // Padrão 1: Barrage das bordas do retângulo
    fn pattern_rectangle_barrage(&mut self, heart: &Heart) {
        if self.attack_timer % 120 == 0 { // A cada 2 segundos
            let new_projectiles = Projectile::create_barrage_from_rectangle(
                20, 70, 120, 70, // Área de jogo (x, y, width, height)
                heart.body,
                8 // 8 projéteis por barrage
            );
            self.projectiles.extend(new_projectiles);
        }
    }
    
    // Padrão 2: Tamanhos mistos - médio/difícil
    fn pattern_mixed_sizes(&mut self, heart: &Heart) {
        if self.attack_timer % 80 == 0 { // A cada ~1.33 segundos
            // Projétil grande lento
            let large_proj = Projectile::new_from_rectangle_edge(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::Large,
                (self.pattern_phase as f32 % 4.0) / 4.0
            );
            self.projectiles.push(large_proj);
            
            self.pattern_phase += 1;
        }
        
        // Projéteis pequenos rápidos intercalados
        if self.attack_timer % 25 == 0 {
            let fast_proj = Projectile::new_from_rectangle_edge(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::FastSmall,
                (self.attack_timer as f32 % 100.0) / 100.0
            );
            self.projectiles.push(fast_proj);
        }
    }
    
    // Padrão 3: Caos dos cantos - difícil
    fn pattern_corner_chaos(&mut self, heart: &Heart) {
        if self.attack_timer % 20 == 0 { // A cada ~0.33 segundos - bem rápido!
            let corners = [
                Point { x: 20, y: 70 },   // Canto superior esquerdo
                Point { x: 140, y: 70 },  // Canto superior direito  
                Point { x: 20, y: 140 },  // Canto inferior esquerdo
                Point { x: 140, y: 140 }, // Canto inferior direito
            ];
            
            let corner_index = (self.attack_timer / 20) % 4;
            let spawn_point = corners[corner_index as usize];
            
            let projectile_type = if self.attack_timer % 60 == 0 {
                ProjectileType::Medium
            } else {
                ProjectileType::Small
            };
            
            let projectile = Projectile::new(
                spawn_point,
                heart.body,
                projectile_type
            );
            self.projectiles.push(projectile);
        }
    }
    
    // Padrão 4: Ataque em espiral - muito difícil
    fn pattern_spiral_attack(&mut self, heart: &Heart) {
        if self.attack_timer % 15 == 0 { // Muito rápido!
            let center_x = 80;
            let center_y = 105; // Centro da área de jogo
            
            // Calcula posição em espiral
            let angle = (self.pattern_phase as f32) * 0.5; // Velocidade da espiral
            let spiral_radius = 30.0 + (self.pattern_phase as f32 % 30.0);
            
            let spawn_x = center_x + (angle.cos() * spiral_radius) as i32;
            let spawn_y = center_y + (angle.sin() * spiral_radius) as i32;
            
            let spawn_point = Point { x: spawn_x, y: spawn_y };
            
            // Alterna entre tipos para maior variação
            let projectile_type = match self.pattern_phase % 3 {
                0 => ProjectileType::Small,
                1 => ProjectileType::FastSmall,
                _ => ProjectileType::Medium,
            };
            
            let projectile = Projectile::new(
                spawn_point,
                heart.body,
                projectile_type
            );
            self.projectiles.push(projectile);
            
            self.pattern_phase += 1;
        }
    }
    
    pub fn draw(&self) {
        // Desenha o Sans
        unsafe { *DRAW_COLORS = 0x03 }
        blit(&SANS, 55, 10, SANS_WIDTH, SANS_HEIGHT, SANS_FLAGS);
        
        // Desenha todos os projéteis
        for proj in &self.projectiles {
            proj.draw();
        }
        
        // Debug: mostra padrão atual (opcional - remova se não quiser)
        unsafe { *DRAW_COLORS = 0x12 }
        let pattern_names = ["BASIC", "BARRAGE", "MIXED", "CHAOS", "SPIRAL"];
        if let Some(name) = pattern_names.get(self.attack_pattern as usize) {
            text(name, 5, 5);
        }
    }
}