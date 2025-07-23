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

#[derive(Clone, Copy, PartialEq)]
pub enum DifficultyLevel {
    Easy,    // 0-30 segundos
    Medium,  // 30-60 segundos  
    Hard,    // 60-90 segundos
    Victory, // Completou tudo!
}

pub struct Enemy {
    #[allow(dead_code)]
    pub pos: Point,
    pub projectiles: Vec<Projectile>,
    game_timer: u32,           // Timer geral do jogo
    attack_timer: u32,         // Timer para ataques
    pub difficulty: DifficultyLevel,
    victory_achieved: bool,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 80, y: 20 },
            projectiles: Vec::new(),
            game_timer: 0,
            attack_timer: 0,
            difficulty: DifficultyLevel::Easy,
            victory_achieved: false,
        }
    }

    pub fn update(&mut self, heart: &mut Heart) {
        self.game_timer += 1;
        self.attack_timer += 1;
        
        // Atualiza nível de dificuldade baseado no tempo
        // 30 segundos = 1800 frames (assumindo 60 FPS)
        self.difficulty = match self.game_timer {
            0..=1800 => DifficultyLevel::Easy,      // 0-30s
            1801..=3600 => DifficultyLevel::Medium, // 30-60s  
            3601..=5400 => DifficultyLevel::Hard,   // 60-90s
            _ => {
                if !self.victory_achieved {
                    self.victory_achieved = true;
                }
                DifficultyLevel::Victory
            }
        };
        
        // Atualiza projéteis existentes
        self.projectiles.retain_mut(|projectile| {
            projectile.update();
            
            // Remove se saiu da tela
            if !projectile.is_on_screen() {
                return false;
            }
            
            // Verifica colisão com o heart
            if projectile.collides_with_heart(heart) {
                let damage = projectile.projectile_type.damage();
                if heart.take_damage(damage) {
                    // Dano foi aplicado com sucesso
                }
                return false; // Remove o projétil após hit
            }
            
            true
        });
        
        // Ataques baseados na dificuldade
        match self.difficulty {
            DifficultyLevel::Easy => self.easy_mode(heart),
            DifficultyLevel::Medium => self.medium_mode(heart),
            DifficultyLevel::Hard => self.hard_mode(heart),
            DifficultyLevel::Victory => {} // Sem ataques - jogador venceu!
        }
    }
    
    // Função para resetar o enemy (para recomeçar jogo)
    pub fn reset(&mut self) {
        self.projectiles.clear();
        self.game_timer = 0;
        self.attack_timer = 0;
        self.difficulty = DifficultyLevel::Easy;
        self.victory_achieved = false;
    }
    
    // MODO FÁCIL: Apenas bolinhas pequenas das bordas, devagar
    fn easy_mode(&mut self, heart: &Heart) {
        if self.attack_timer % 80 == 0 { // A cada ~1.33 segundos
            // Usa um multiplicador maior para criar mais variação nas posições
            let edge_position = ((self.attack_timer as f32 * 0.37) % 4.0) / 4.0; // 0.37 é um multiplicador que cria variação
            
            let projectile = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::Small, // Apenas pequenos
                edge_position,
                20 // 20 pixels de distância da borda
            );
            self.projectiles.push(projectile);
        }
    }
    
    // MODO MÉDIO: Bolinhas pequenas + rápidas + quadrados grandes (misto)
    fn medium_mode(&mut self, heart: &Heart) {
        // Projéteis pequenos regulares
        if self.attack_timer % 60 == 0 { // A cada 1 segundo
            let edge_position = ((self.attack_timer as f32 * 0.43) % 4.0) / 4.0;
            
            let projectile = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::Small,
                edge_position,
                18 // 18 pixels de distância
            );
            self.projectiles.push(projectile);
        }
        
        // Projéteis grandes ocasionais
        if self.attack_timer % 120 == 0 { // A cada 2 segundos
            let edge_position = ((self.attack_timer as f32 * 0.71) % 4.0) / 4.0;
            
            let large_proj = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::Large, // Grandes e lentos
                edge_position,
                25 // Maior distância para projéteis grandes
            );
            self.projectiles.push(large_proj);
        }
        
        // Projéteis rápidos ocasionais
        if self.attack_timer % 90 == 0 { // A cada 1.5 segundos
            let edge_position = ((self.attack_timer as f32 * 0.29) % 4.0) / 4.0;
            
            let fast_proj = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::FastSmall, // Pequenos e rápidos
                edge_position,
                15 // Menos distância para rápidos (mais desafiador)
            );
            self.projectiles.push(fast_proj);
        }
    }
    
    // MODO DIFÍCIL: Barrage leve + ataques variados
    fn hard_mode(&mut self, heart: &Heart) {
        // Projéteis constantes das bordas
        if self.attack_timer % 45 == 0 { // A cada 0.75 segundos
            let edge_position = ((self.attack_timer as f32 * 0.61) % 4.0) / 4.0;
            
            let projectile_type = match (self.attack_timer / 45) % 3 {
                0 => ProjectileType::Small,
                1 => ProjectileType::Medium,
                _ => ProjectileType::FastSmall,
            };
            
            let projectile = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                projectile_type,
                edge_position,
                16 // Distância menor no modo difícil
            );
            self.projectiles.push(projectile);
        }
        
        // Barrage leve ocasional (4 projéteis ao invés de 8)
        if self.attack_timer % 180 == 0 { // A cada 3 segundos
            let new_projectiles = Projectile::create_barrage_from_rectangle_with_distance(
                20, 70, 120, 70,
                heart.body,
                4, // Apenas 4 projéteis por barrage (mais leve)
                22 // Distância da barrage
            );
            self.projectiles.extend(new_projectiles);
        }
        
        // Projétil grande ocasional
        if self.attack_timer % 150 == 0 { // A cada 2.5 segundos
            let edge_position = ((self.attack_timer as f32 * 0.83) % 4.0) / 4.0;
            
            let large_proj = Projectile::new_from_rectangle_edge_with_distance(
                20, 70, 120, 70,
                heart.body,
                ProjectileType::Large,
                edge_position,
                20 // Distância média para grandes
            );
            self.projectiles.push(large_proj);
        }
    }
    
    // Função para verificar se o jogador venceu
    pub fn has_player_won(&self) -> bool {
        self.victory_achieved
    }
    
    // Função para obter tempo restante no nível atual (em segundos)
    pub fn get_time_remaining_in_level(&self) -> u32 {
        let frames_per_level = 1800; // 30 segundos
        let current_level_frames = self.game_timer % frames_per_level;
        let remaining_frames = frames_per_level - current_level_frames;
        remaining_frames / 60 // Converte para segundos
    }
    
    // Função para obter tempo total de jogo (em segundos)
    pub fn get_total_time(&self) -> u32 {
        self.game_timer / 60
    }
    
    pub fn draw(&self) {
        // Desenha o Sans
        unsafe { *DRAW_COLORS = 0x03 }
        blit(&SANS, 55, 10, SANS_WIDTH, SANS_HEIGHT, SANS_FLAGS);
        
        // Desenha todos os projéteis
        for proj in &self.projectiles {
            proj.draw();
        }
        
        // Mostra nível atual e tempo
        unsafe { *DRAW_COLORS = 0x12 }
        let (level_name, time_remaining) = match self.difficulty {
            DifficultyLevel::Easy => ("EASY", self.get_time_remaining_in_level()),
            DifficultyLevel::Medium => ("MEDIUM", self.get_time_remaining_in_level()),
            DifficultyLevel::Hard => ("HARD", self.get_time_remaining_in_level()),
            DifficultyLevel::Victory => ("VICTORY!", 0),
        };
        
        text(level_name, 5, 5);
        
        if !matches!(self.difficulty, DifficultyLevel::Victory) {
            // Mostra tempo restante no nível
            let time_text = format!("{}s", time_remaining);
            text(&time_text, 5, 15);
        } else {
            text("COMPLETED!", 5, 15);
        }
    }
}