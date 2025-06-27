use wasm_bindgen::prelude::*;

// Gravity
//
// applying this: pos_y += velocity_y * delta

// on_ground
// terminal_velocity for maximum speed the sprite can get to

// on jump velocity_y = -jump_force
// Only allow jumping if on_ground == true
#[wasm_bindgen]
pub struct Sprite {
    x: f32,
    y: f32,
    frame_index: u32,
    width: f32,
    height: f32,
    velocity_y: f32,
    velocity_x: f32,
    on_ground: bool,
    terminal_velocity: f32,
}

#[wasm_bindgen]
pub struct Block {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    velocity_y: f32,
}

#[wasm_bindgen]
impl Sprite {
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f32,
        y: f32,
        frame_index: u32,
        width: f32,
        height: f32,
        velocity_y: f32,
        velocity_x: f32,
        on_ground: bool,
        terminal_velocity: f32,
    ) -> Sprite {
        Sprite {
            x,
            y,
            frame_index,
            width,
            height,
            velocity_y,
            velocity_x,
            on_ground,
            terminal_velocity,
        }
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
static mut VELOCITY_Y: f32 = 0.0;
static mut VELOCITY_X: f32 = 0.0;
static mut ON_GROUND: bool = false;
static TERMINAL_VELOCITY: f32 = 5.0;

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

// Should be jump!
// just go ahead and jump!
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
    unsafe {
        Sprite::new(
            POS_X,
            POS_Y,
            FRAME_INDEX,
            32.0,
            32.0,
            VELOCITY_Y,
            VELOCITY_X,
            ON_GROUND,
            TERMINAL_VELOCITY,
        )
    }
}

#[wasm_bindgen]
pub fn check_collision(sprite: &Sprite, x: f32, y: f32) {
    let sprite_x = sprite.x;
    let sprite_y = sprite.y;
    if sprite_x - 32.0 == x || sprite_y - 32.0 == y {
        print!("Collision?!")
    }
}

#[wasm_bindgen]
pub fn check_block_collision(sprite: &Sprite, block: &Block) -> bool {
    let sx = sprite.x;
    let sy = sprite.y;
    let sh = sprite.height;
    let sw = sprite.width;

    let bx = block.x;
    let by = block.y;
    let bw = block.width;
    let bh = block.height;

    // AABB collision check
    sx < bx + bw && sx + sw > bx && sy < by + bh && sy + sh > by
}
