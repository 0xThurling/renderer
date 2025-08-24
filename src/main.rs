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
use sdl2_sys::SDL_Delay;
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

use crate::display::draw_line;
use crate::display::draw_triangle;
use crate::mesh::MESH_FACES;
use crate::mesh::MESH_VERTICES;
use crate::mesh::N_MESH_FACES;
use crate::triangle::Triangle;

extern crate sdl2_sys;

#[macro_use]
mod macros;

mod display;
mod mesh;
mod triangle;
mod vector;

const FOV_FACTOR: f32 = 640.0;
const ZERO_VECTOR3: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const ZERO_VECTOR2: Vector2 = Vector2 { x: 0.0, y: 0.0 };
const CAMERA: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -5.0,
};

// Set empty triangles
const ZERO_TRIANGLE: Triangle = Triangle {
    points: [ZERO_VECTOR2; 3],
};
static mut TRIANGLES_TO_RENDER: [Triangle; N_MESH_FACES as usize] =
    [ZERO_TRIANGLE; N_MESH_FACES as usize];

static mut CUBE_ROTATION: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

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
    Vector2::new(
        (FOV_FACTOR * vector.x) / vector.z,
        (FOV_FACTOR * vector.y) / vector.z,
    )
}

// Function for othographic projections
fn orthographic_project(vector: &Vector3) -> Vector2 {
    Vector2::new(
        (128.0 * vector.x), 
        (128.0 * vector.y)
    )
}

fn update() {
    unsafe {
        // NOTE: Check if the timer has passed our specified values
        let time_to_wait = FRAME_TARGET_TIME - (SDL_GetTicks() - PREVIOUS_FRAME_TIME) as i32;

        if time_to_wait > 0 && time_to_wait <= FRAME_TARGET_TIME {
            SDL_Delay(time_to_wait as u32);
        }

        PREVIOUS_FRAME_TIME = SDL_GetTicks();

        CUBE_ROTATION.x += 0.01;
        CUBE_ROTATION.y += 0.01;
        CUBE_ROTATION.z += 0.01;

        for i in 0..N_MESH_FACES {
            let face = &MESH_FACES[i as usize];

            let face_vertices: [Vector3; 3] = [
                MESH_VERTICES[(face.a - 1) as usize].clone(),
                MESH_VERTICES[(face.b - 1) as usize].clone(),
                MESH_VERTICES[(face.c - 1) as usize].clone(),
            ];

            let mut projected_triangle = ZERO_TRIANGLE;

            // Loop all three vertices of this current face and apply transformations
            for j in 0..3 {
                let transformed_vertex = &face_vertices[j as usize];
                let point_camera_pov = Vector3::new(
                    transformed_vertex.x,
                    transformed_vertex.y,
                    transformed_vertex.z,
                );

                let mut transformed_vertex = point_camera_pov.rotate_x(CUBE_ROTATION.x);
                transformed_vertex = transformed_vertex.rotate_y(CUBE_ROTATION.y);
                transformed_vertex = transformed_vertex.rotate_z(CUBE_ROTATION.z);

                // NOTE: This might be needed depending on the view of the cube mesh
                // Translate the vertices away from the camera
                transformed_vertex.z -= CAMERA.z;

                let mut projected_point = project(&transformed_vertex);

                // NOTE: Scale and translate point to the middle of the screen
                projected_point.x += (WINDOW_WIDTH / 2) as f32;
                projected_point.y += (WINDOW_HEIGHT / 2) as f32;

                projected_triangle.points[j] = projected_point;
            }

            // Save projected triangle in the array of triangles to render 
            TRIANGLES_TO_RENDER[i as usize] = projected_triangle;
        }
    }
}

fn render() {
    unsafe {
        // Must clear before drawing
        clear_color_buffer(0xFF000000);
        SDL_SetRenderDrawColor(SDL_RENDERER, 255, 0, 0, 255);
        SDL_RenderClear(SDL_RENDERER);

        // NOTE: Loop projected triangles and render them
        for i in 0..N_MESH_FACES {
            let triangle = &TRIANGLES_TO_RENDER[i as usize];

            draw_rect(triangle.points[0].x as i32, triangle.points[0].y as i32, 3, 3, 0xFF00CC00);
            draw_rect(triangle.points[1].x as i32, triangle.points[1].y as i32, 3, 3, 0xFF00CC00);
            draw_rect(triangle.points[2].x as i32, triangle.points[2].y as i32, 3, 3, 0xFF00CC00);

            draw_triangle(
                triangle.points[0].x as i32,
                triangle.points[0].y as i32, 
                triangle.points[1].x as i32,
                triangle.points[1].y as i32,
                triangle.points[2].x as i32,
                triangle.points[2].y as i32,
                0xFF00CC00
            );          
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
