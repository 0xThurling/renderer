use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use display::COLOR_BUFFER;
use display::FRAME_TARGET_TIME;
use display::SDL_RENDERER;
use display::SDL_TEXTURE;
use display::WINDOW_HEIGHT;
use display::WINDOW_WIDTH;
use display::clear_color_buffer;
use display::draw_rect;
use display::initialise_window;
use display::render_color_buffer;
use display::shutdown;
use sdl2_sys::SDL_CreateTexture;
use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_EventType;
use sdl2_sys::SDL_GetTicks;
use sdl2_sys::SDL_KeyCode;
use sdl2_sys::SDL_PixelFormatEnum;
use sdl2_sys::SDL_PollEvent;
use sdl2_sys::SDL_RenderClear;
use sdl2_sys::SDL_RenderPresent;
use sdl2_sys::SDL_SetRenderDrawColor;
use sdl2_sys::SDL_TextureAccess;
use vector::Vector2;
use vector::Vector3;

extern crate sdl2_sys;

#[macro_use]
mod macros;

mod display;
mod vector;

const FOV_FACTOR: f32 = 640.0;
const N_POINTS: i32 = 9 * 9 * 9;
const ZERO_VECTOR3: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const ZERO_VECTOR2: Vector2 = Vector2 { x: 0.0, y: 0.0 };

const CAMERA: Vector3 = Vector3 {x: 0.0, y: 0.0, z: -5.0};
static mut CUBE_ROTATION: Vector3 = Vector3 {x: 0.0, y: 0.0, z: 0.0};

static mut CUBE_POINTS: [Vector3; N_POINTS as usize] = [ZERO_VECTOR3; N_POINTS as usize];
static mut PROJECTED_POINTS: [Vector2; N_POINTS as usize] = [ZERO_VECTOR2; N_POINTS as usize];

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

static mut PREVIOUS_FRAME_TIME: u32 = 0;

fn setup() {
    // Allocate the memory needed for the COLOR_BUFFER
    let mut buffer_option = COLOR_BUFFER
        .lock()
        .expect("Failed to get the lock of the COLOR_BUFFER");

    unsafe {
        *buffer_option = Some(vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize]);

        // Creating the SDL texture that is used to display the color
        SDL_TEXTURE = SDL_CreateTexture(
            SDL_RENDERER,
            SDL_PixelFormatEnum::SDL_PIXELFORMAT_ARGB8888 as u32,
            SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );

        let mut point_count = 0;

        // X-LOOP
        let mut x = -1.0;
        while x <= 1.0 {
            // Y-LOOP
            let mut y = -1.0;
            while y <= 1.0 {
                // Z-LOOP
                let mut z = -1.0;
                while z <= 1.0 {
                    let vec = Vector3::new(x, y, z);

                    CUBE_POINTS[point_count] = vec;
                    point_count += 1; // NOTE: This might break

                    z += 0.25;
                }
                y += 0.25;
            }
            x += 0.25;
        }
    }
}

fn process_input() {
    unsafe {
        // REMEBER TO USE THIS PATTERN WHEN CREATING A MUTABLE REFERENCE
        let mut event: SDL_Event = std::mem::zeroed();
        SDL_PollEvent(&mut event);

        match event.type_ {
            x if x == SDL_EventType::SDL_QUIT as u32 => {
                IS_RUNNING.store(false, Ordering::SeqCst);
            }
            x if x == SDL_EventType::SDL_KEYDOWN as u32 => {
                if event.key.keysym.sym == SDL_KeyCode::SDLK_ESCAPE as i32 {
                    IS_RUNNING.store(false, Ordering::SeqCst);
                }
            }
            _ => {}
        }
    }
}

// Returns a vector ignoring the z-axis
fn project(vector: &Vector3) -> Vector2 {
    Vector2::new((FOV_FACTOR * vector.x) / vector.z, (FOV_FACTOR * vector.y) / vector.z)
}

fn update() {
    unsafe {
        // NOTE: Check if the timer has passed our specified values
        while !sdl_ticks_passed!(SDL_GetTicks(), PREVIOUS_FRAME_TIME + FRAME_TARGET_TIME as u32) {}

        PREVIOUS_FRAME_TIME = SDL_GetTicks();

        CUBE_ROTATION.x += 0.01;
        CUBE_ROTATION.y += 0.01;
        CUBE_ROTATION.z += 0.01;

        for i in 0..N_POINTS {
            let point = &CUBE_POINTS[i as usize];
            let point_camera_pov = Vector3::new(point.x, point.y, point.z);

            let mut transformed_point = point_camera_pov.rotate_x(CUBE_ROTATION.x);
            transformed_point = transformed_point.rotate_y(CUBE_ROTATION.y);
            transformed_point = transformed_point.rotate_z(CUBE_ROTATION.z);

            transformed_point.z -= CAMERA.z;

            // Save the projected 2D points in an array of projected points
            PROJECTED_POINTS[i as usize] = project(&transformed_point);
        }
    }
}

fn render() {
    unsafe {
        // Must clear before drawing
        clear_color_buffer(0xFF000000);
        SDL_SetRenderDrawColor(SDL_RENDERER, 255, 0, 0, 255);
        SDL_RenderClear(SDL_RENDERER);

        for i in 0..N_POINTS {
            let point = &PROJECTED_POINTS[i as usize];

            let point_x_translated: i32 = point.x as i32 + (WINDOW_WIDTH / 2);
            let point_y_translated: i32 = point.y as i32 + (WINDOW_HEIGHT / 2);

            draw_rect(point_x_translated, point_y_translated, 4, 4, 0xFF00FFFF);
        }

        render_color_buffer();

        SDL_RenderPresent(SDL_RENDERER);
    }
}

fn main() {
    IS_RUNNING.store(initialise_window(), Ordering::SeqCst);

    setup();

    while IS_RUNNING.load(Ordering::SeqCst) {
        process_input();
        update();
        render();
    }

    shutdown();
}
