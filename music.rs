use crate::wasm4::*;

#[derive(Clone, Copy)]
struct Note {
    note_id: i32,     // número da nota 
    duration: u32,    // em frames
    volume: u32,      // volume
    flags: u32,       // "Instrumento" usado
    wait: u32,        // tempo até a próxima nota
}

fn note_to_freq(note_id: i32) -> u32 {
    if note_id < 0 {
        return 0;
    }
    let midi = note_id + 11;
    let freq = 440.0 * f64::powf(2.0, (midi - 69) as f64 / 12.0);
    freq.round() as u32
}

// === Megalovania ===
// Melodia no canal 1 (Pulse 1)
static MELODY: &[Note] = &[
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 49, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 49, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 48, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 48, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 47, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 47, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },






    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 54, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 56, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 57, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 61, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 58, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 61, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: 75, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },






    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 75, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 75, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 70, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 66, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: 73, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 68, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: 66, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 65, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: 59, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 63, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 65, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 66, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: 75, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },



    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE2, wait: 7 },
];






// Harmonia no canal 2 (Pulse 2)
static HARMONY: &[Note] = &[
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },






    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },






    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 46, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 45, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 37, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 46, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 45, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 36, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 46, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 45, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 35, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 51, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    
    Note { note_id: 46, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 45, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },

    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 39, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 42, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
    Note { note_id: 44, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },



    Note { note_id: -1, duration: 10, volume: 50, flags: TONE_PULSE1, wait: 7 },
];

static mut IDX1: usize = 0;
static mut TIMER1: u32 = 0;
static mut IDX2: usize = 0;
static mut TIMER2: u32 = 0;

pub fn update_music() {
    unsafe {
        // Canal 1 - Melodia
        if IDX1 < MELODY.len() {
            if TIMER1 == 0 {
                let n = &MELODY[IDX1];
                if n.note_id >= 0 {
                    let freq = note_to_freq(n.note_id);
                    tone(freq, n.duration, n.volume, n.flags);
                }
                TIMER1 = n.wait;
                IDX1 += 1;
            } else {
                TIMER1 -= 1;
            }
        }

        // Canal 2 - Harmonia
        if IDX2 < HARMONY.len() {
            if TIMER2 == 0 {
                let n = &HARMONY[IDX2];
                if n.note_id >= 0 {
                    let freq = note_to_freq(n.note_id);
                    tone(freq, n.duration, n.volume, n.flags);
                }
                TIMER2 = n.wait;
                IDX2 += 1;
            } else {
                TIMER2 -= 1;
            }
        }

        // Reinicia a música
        if IDX1 >= MELODY.len() && IDX2 >= HARMONY.len() {
            IDX1 = 0;
            TIMER1 = 0;
            IDX2 = 0;
            TIMER2 = 0;
        }
    }
}