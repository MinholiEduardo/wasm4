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
    pub life: u8, // de 0 a 5    
}

impl Heart {
    pub fn new() -> Self {
        Self {
            body: Point {
                x: 20 + (120 - 9) / 2, // centralizado no eixo X
                y: 70 + (70 - 9) / 2,  // centralizado no eixo Y
            },
            life: 5,
        }
    }
    pub fn update(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };

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

        // Mantém o coração dentro da área jogável (ajuste os limites conforme necessário)
        self.body.x = self.body.x.clamp(21, 130);
        self.body.y = self.body.y.clamp(71, 130);
    }

    pub fn draw(&self) {

        unsafe { *DRAW_COLORS = 0x14 }
        blit(&HEART, self.body.x, self.body.y, HEART_WIDTH, HEART_HEIGHT, HEART_FLAGS);
    }


    pub fn draw_life_bar(&self) {
        let bar_x: i32 = 50;
        let bar_y: i32 = 145;
        let segment_width: u32 = 10;
        let spacing: u32 = 2;

        for i in 0..5u8 {
            unsafe {
                *DRAW_COLORS = if i < self.life { 0x41 } else { 0x21 };
            }

            let x = bar_x + (i as i32) * ((segment_width + spacing) as i32);
            rect(x, bar_y, segment_width, 5);
        }
    }

    pub fn is_hit(&self, px: i32, py: i32) -> bool {
        let hx = self.body.x;
        let hy = self.body.y;
        let hw = HEART_WIDTH as i32;
        let hh = HEART_HEIGHT as i32;

        px >= hx && px < hx + hw && py >= hy && py < hy + hh
    }
    
}
