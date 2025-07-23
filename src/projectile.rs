use crate::wasm4::*;
use crate::heart::Point;

pub struct Projectile {
    pub pos: (f32, f32),
    pub velocity: (f32, f32),
}

impl Projectile {
    pub fn new(from: Point, to: Point) -> Self {
        let dx = (to.x - from.x) as f32;
        let dy = (to.y - from.y) as f32;
        let distance = (dx * dx + dy * dy).sqrt().max(1.0); // evita divisão por 0

        let speed = 1.5;
        let velocity = (dx / distance * speed, dy / distance * speed);

        Self {
            pos: (from.x as f32, from.y as f32),
            velocity,
        }
    }

    pub fn update(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
    }

    pub fn draw(&self) {
        unsafe { *DRAW_COLORS = 0x13 }
        oval(self.pos.0 as i32, self.pos.1 as i32, 5, 5);
    }

    pub fn is_on_screen(&self) -> bool {
        let (x, y) = self.pos;
        x >= 0.0 && x <= 160.0 && y >= 0.0 && y <= 160.0
    }    
}
