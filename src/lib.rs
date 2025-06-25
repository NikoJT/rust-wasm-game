use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sprite {
    x: f32,
    y: f32,
    frame_index: u32,
}

#[wasm_bindgen]
impl Sprite {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, frame_index: u32) -> Sprite {
        Sprite { x, y, frame_index }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn frame_index(&self) -> u32 {
        self.frame_index
    }
}

static mut POS_X: f32 = 0.0;
static mut POS_Y: f32 = 0.0;
static mut FRAME_INDEX: u32 = 0;
static mut FRAME_TIMER: f32 = 0.0;

const FRAME_TIME: f32 = 0.2;

#[wasm_bindgen]
pub fn move_right(delta: f32) {
    unsafe {
        POS_X += 50.0 * delta;

        if POS_X > 800.0 {
            POS_X = -32.0;
        }

        FRAME_TIMER += delta;
        if FRAME_TIMER >= FRAME_TIME {
            FRAME_TIMER = 0.0;
            FRAME_INDEX = (FRAME_INDEX + 1) % 6
        }
    }
}

#[wasm_bindgen]
pub fn move_down(delta: f32) {
    unsafe {
        POS_Y += 50.0 * delta;

        if POS_Y > 600.0 {
            POS_Y = -32.0;
        }

        FRAME_TIMER += delta;
        if FRAME_TIMER >= FRAME_TIME {
            FRAME_TIMER = 0.0;
            FRAME_INDEX = (FRAME_INDEX + 1) % 6
        }
    }
}

#[wasm_bindgen]
pub fn move_left(delta: f32) {
    unsafe {
        POS_X -= 50.0 * delta;

        if POS_X < -32.0 {
            POS_X = 800.0;
        }

        FRAME_TIMER += delta;
        if FRAME_TIMER >= FRAME_TIME {
            FRAME_TIMER = 0.0;
            FRAME_INDEX = (FRAME_INDEX + 1) % 6
        }
    }
}

#[wasm_bindgen]
pub fn move_up(delta: f32) {
    unsafe {
        POS_Y -= 50.0 * delta;

        if POS_Y < -32.0 {
            POS_Y = 600.0;
        }

        FRAME_TIMER += delta;
        if FRAME_TIMER >= FRAME_TIME {
            FRAME_TIMER = 0.0;
            FRAME_INDEX = (FRAME_INDEX + 1) % 6
        }
    }
}

#[wasm_bindgen]
pub fn get_sprite_position() -> Sprite {
    unsafe { Sprite::new(POS_X, POS_Y, FRAME_INDEX) }
}
