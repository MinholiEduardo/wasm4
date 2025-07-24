use crate::wasm4::*;
use crate::heart::{Point, Heart};

#[derive(Clone, Copy)]
pub enum ProjectileType {
    Small,
    Medium,
    Large,
    FastSmall,
}

impl ProjectileType {
    pub fn size(&self) -> u32 {
        match self {
            ProjectileType::Small => 4,
            ProjectileType::Medium => 8,
            ProjectileType::Large => 12,
            ProjectileType::FastSmall => 3,
        }
    }

    pub fn hitbox_size(&self) -> u32 {
        match self {
            ProjectileType::Small => 4,
            ProjectileType::Medium => 7,     // Ligeiramente menor
            ProjectileType::Large => 9,      // Bem menor que os 12 visuais
            ProjectileType::FastSmall => 3,
        }
    }
    
    pub fn speed(&self) -> f32 {
        match self {
            ProjectileType::Small => 1.2,
            ProjectileType::Medium => 1.0,
            ProjectileType::Large => 0.8,
            ProjectileType::FastSmall => 2.0,
        }
    }
    
    pub fn color(&self) -> u16 {
        match self {
            ProjectileType::Small => 0x13,      // Cor padrão
            ProjectileType::Medium => 0x23,     // Cor diferente para médios
            ProjectileType::Large => 0x33,      // Cor diferente para grandes
            ProjectileType::FastSmall => 0x43,  // Cor diferente para rápidos
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            ProjectileType::Small => 5,        // Dano baixo
            ProjectileType::Medium => 10,      // Dano médio
            ProjectileType::Large => 20,       // Dano alto
            ProjectileType::FastSmall => 5,    // Dano baixo (compensa pela velocidade)
        }
    }
}

pub struct Projectile {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
    pub projectile_type: ProjectileType,
}

impl Projectile {
    pub fn new(from: Point, to: Point, projectile_type: ProjectileType) -> Self {
        let dx = (to.x - from.x) as f32;
        let dy = (to.y - from.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt().max(1.0);
        let speed = projectile_type.speed();
        let velocity = (dx / distance * speed, dy / distance * speed);
        
        Self {
            pos: (from.x as f32, from.y as f32),
            velocity,
            projectile_type,
        }
    }
    
    // Função para criar projétil de uma borda específica do retângulo com distância
    pub fn new_from_rectangle_edge_with_distance(
        rect_x: i32, 
        rect_y: i32, 
        rect_width: u32, 
        rect_height: u32,
        target: Point,
        projectile_type: ProjectileType,
        edge_position: f32,  // 0.0 a 1.0 para posição na borda
        distance: i32        // Distância da borda em pixels
    ) -> Self {
        // Escolhe uma borda aleatória (0-3: top, right, bottom, left)
        let edge = (edge_position * 4.0) as u32 % 4;
        let position_on_edge = edge_position * 4.0 - (edge as f32);
        
        let spawn_point = match edge {
            0 => {
                // Borda superior - spawn acima do retângulo
                let x = rect_x + (rect_width as f32 * position_on_edge) as i32;
                Point { x, y: rect_y - distance }
            },
            1 => {
                // Borda direita - spawn à direita do retângulo
                let y = rect_y + (rect_height as f32 * position_on_edge) as i32;
                Point { x: rect_x + rect_width as i32 + distance, y }
            },
            2 => {
                // Borda inferior - spawn abaixo do retângulo
                let x = rect_x + (rect_width as f32 * position_on_edge) as i32;
                Point { x, y: rect_y + rect_height as i32 + distance }
            },
            _ => {
                // Borda esquerda - spawn à esquerda do retângulo
                let y = rect_y + (rect_height as f32 * position_on_edge) as i32;
                Point { x: rect_x - distance, y }
            }
        };
        
        Self::new(spawn_point, target, projectile_type)
    }
    
    // Função para criar múltiplos projéteis das bordas com distância
    pub fn create_barrage_from_rectangle_with_distance(
        rect_x: i32,
        rect_y: i32, 
        rect_width: u32,
        rect_height: u32,
        target: Point,
        count: u32,
        distance: i32
    ) -> Vec<Self> {
        let mut projectiles = Vec::new();
        
        for i in 0..count {
            let edge_position = (i as f32) / (count as f32);
            
            // Varia os tipos de projétil
            let projectile_type = match i % 4 {
                0 => ProjectileType::Small,
                1 => ProjectileType::Medium,
                2 => if i % 8 == 2 { ProjectileType::Large } else { ProjectileType::Small },
                _ => if i % 12 == 3 { ProjectileType::FastSmall } else { ProjectileType::Small },
            };
            
            let projectile = Self::new_from_rectangle_edge_with_distance(
                rect_x, rect_y, rect_width, rect_height,
                target, projectile_type, edge_position, distance
            );
            
            projectiles.push(projectile);
        }
        
        projectiles
    }
    
    pub fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
    }
    
    pub fn draw(&self) {
        unsafe { 
            *DRAW_COLORS = self.projectile_type.color();
        }
        
        let size = self.projectile_type.size();
        let x = self.pos.0 as i32 - (size as i32 / 2);
        let y = self.pos.1 as i32 - (size as i32 / 2);
        
        // Desenha como círculo para projéteis pequenos e médios
        // Para projéteis grandes, pode desenhar como retângulo para parecer mais ameaçador
        match self.projectile_type {
            ProjectileType::Large => {
                rect(x, y, size, size);
            },
            _ => {
                oval(x, y, size, size);
            }
        }
    }
    
    pub fn is_on_screen(&self) -> bool {
        let (x, y) = self.pos;
        let size = self.projectile_type.size() as f32;
        x >= -size && x <= 160.0 + size && y >= -size && y <= 160.0 + size
    }
    
    // Função para verificar colisão com o heart
    pub fn collides_with_heart(&self, heart: &Heart) -> bool {
        // Usa hitbox_size ao invés de size para colisão mais justa
        let projectile_hitbox = self.projectile_type.hitbox_size() as i32;
        let heart_width = 9i32;  // HEART_WIDTH  
        let heart_height = 9i32; // HEART_HEIGHT
        
        // Centro do projétil
        let proj_center_x = self.pos.0 as i32;
        let proj_center_y = self.pos.1 as i32;
        
        // Calcula as bordas da hitbox do projétil (centralizada)
        let proj_left = proj_center_x - projectile_hitbox / 2;
        let proj_right = proj_center_x + projectile_hitbox / 2;
        let proj_top = proj_center_y - projectile_hitbox / 2;
        let proj_bottom = proj_center_y + projectile_hitbox / 2;
        
        // Bordas do coração
        let heart_left = heart.body.x;
        let heart_right = heart.body.x + heart_width;
        let heart_top = heart.body.y;
        let heart_bottom = heart.body.y + heart_height;
        
        // Verifica sobreposição dos retângulos
        proj_left < heart_right &&
        proj_right > heart_left &&
        proj_top < heart_bottom &&
        proj_bottom > heart_top
    }
    
    // Função para verificar se está dentro da área de jogo (baseado no seu retângulo)
    pub fn is_inside_game_area(&self) -> bool {
        let (x, y) = self.pos;
        x >= 20.0 && x <= 140.0 && y >= 70.0 && y <= 140.0
    }
}