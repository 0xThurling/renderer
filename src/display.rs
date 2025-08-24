use std::{ffi::c_void, ptr::null, sync::Mutex};

use sdl2_sys::{
    SDL_DestroyRenderer, SDL_DestroyWindow, SDL_DisplayMode, SDL_GetCurrentDisplayMode,
    SDL_GetError, SDL_INIT_EVERYTHING, SDL_Quit, SDL_RenderCopy, SDL_Renderer,
    SDL_SetWindowFullscreen, SDL_Texture, SDL_UpdateTexture, SDL_WINDOWPOS_CENTERED_MASK,
    SDL_Window, SDL_WindowFlags,
};

pub const FPS: i32 = 60;
pub const FRAME_TARGET_TIME: i32 = 1000 / FPS;

pub static mut WINDOW_WIDTH: i32 = 800;
pub static mut WINDOW_HEIGHT: i32 = 600;

// unsafe code global variables
pub static mut SDL_WINDOW: *mut SDL_Window = std::ptr::null_mut();
pub static mut SDL_RENDERER: *mut SDL_Renderer = std::ptr::null_mut();
pub static mut SDL_TEXTURE: *mut SDL_Texture = std::ptr::null_mut();

pub static COLOR_BUFFER: Mutex<Option<Vec<u32>>> = Mutex::new(None);

pub fn initialise_window() -> bool {
    unsafe {
        let result = sdl2_sys::SDL_Init(SDL_INIT_EVERYTHING);

        // Checks if the initialisation failed
        assert!(
            result == 0,
            "SDL init failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        // Use SDL to query what is the max screen width and height
        let mut display_mode: SDL_DisplayMode = std::mem::zeroed();
        let result = SDL_GetCurrentDisplayMode(0, &mut display_mode);

        assert!(
            result == 0,
            "SDL_GetCurrentDisplayMode failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        WINDOW_WIDTH = display_mode.w;
        WINDOW_HEIGHT = display_mode.h;

        // Create SDL WINDOW
        SDL_WINDOW = sdl2_sys::SDL_CreateWindow(
            std::ptr::null(),
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            sdl2_sys::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32,
        );

        // Checks if the window was initialised
        assert!(
            !SDL_WINDOW.is_null(),
            "SDL_CreateWindow failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        SDL_RENDERER = sdl2_sys::SDL_CreateRenderer(SDL_WINDOW, -1, 0);

        // Checks if the window was initialised
        assert!(
            !SDL_RENDERER.is_null(),
            "SDL_CreateRenderer failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        let result =
            SDL_SetWindowFullscreen(SDL_WINDOW, SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32);

        assert!(
            result == 0,
            "SDL_SetWindowFullscreen failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );
    }

    true
}

pub fn clear_color_buffer(color: u32) {
    let mut buffer_option = COLOR_BUFFER
        .lock()
        .expect("Failed to acquire lock for the COLOR_BUFFER");

    unsafe {
        if let Some(buffer) = buffer_option.as_mut() {
            for y in 0..WINDOW_HEIGHT {
                for x in 0..WINDOW_WIDTH {
                    // Gets the cell (pixel) on the screen by row + col
                    buffer[((WINDOW_WIDTH * y) + x) as usize] = color;
                }
            }
        }
    }
}

pub fn render_color_buffer() {
    let mut buffer_option = COLOR_BUFFER
        .lock()
        .expect("Failed to acquire lock for the COLOR_BUFFER");

    if let Some(buffer) = buffer_option.as_mut() {
        unsafe {
            SDL_UpdateTexture(
                SDL_TEXTURE,
                null(),
                buffer.as_ptr() as *const c_void,
                ((WINDOW_WIDTH as usize) * size_of::<u32>()) as i32,
            );

            SDL_RenderCopy(SDL_RENDERER, SDL_TEXTURE, null(), null());
        }
    }
}

pub fn draw_grid() {
    let scale = 20;

    let mut buffer_option = COLOR_BUFFER
        .lock()
        .expect("Failed to acquire lock for the COLOR_BUFFER");

    unsafe {
        if let Some(buffer) = buffer_option.as_mut() {
            for y in 0..WINDOW_HEIGHT {
                for x in 0..WINDOW_WIDTH {
                    if y % scale == 0 || x % scale == 0 {
                        buffer[((WINDOW_WIDTH * y) + x) as usize] = 0xFF333333;
                    }
                }
            }
        }
    }
}

pub fn draw_triangle(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
   draw_line(x0, y0, x1, y1, color); 
   draw_line(x1, y1, x2, y2, color); 
   draw_line(x2, y2, x0, y0, color); 
}

pub fn draw_rect(x_pos: i32, y_pos: i32, width: u32, height: u32, color: u32) {
    for y in 0..height {
        for x in 0..width {
            let current_x = x_pos + x as i32;
            let current_y = y_pos + y as i32;
            draw_pixel(current_x, current_y, color);
        }
    }
}

pub fn draw_pixel(x: i32, y: i32, color: u32) {
    unsafe {
        if x >= 0 && x < WINDOW_WIDTH as i32 && y >= 0 && y < WINDOW_HEIGHT as i32 {
            let mut buffer_option = COLOR_BUFFER
                .lock()
                .expect("Failed to acquire lock for the COLOR_BUFFER");

            if let Some(buffer) = buffer_option.as_mut() {
                buffer[((WINDOW_WIDTH as i32 * y) + x) as usize] = color;
            }
        }
    }
}

/// Draws a line between two x,y points
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let delta_x = x1 - x0;
    let delta_y = y1 - y0;

    let longest_length_side = if i32::abs(delta_x) >= i32::abs(delta_y) {
        i32::abs(delta_x)
    } else {
        i32::abs(delta_y)
    };

    let x_inc = delta_x as f32 / longest_length_side as f32;
    let y_inc = delta_y as f32 / longest_length_side as f32;

    let mut current_x = x0 as f32;
    let mut current_y = y0 as f32;

    for _ in 0..=longest_length_side {
        // Gets the rounded value fo the pixel coordinate on screen
        draw_pixel(
            f32::round(current_x) as i32,
            f32::round(current_y) as i32,
            color,
        );

        current_x += x_inc;
        current_y += y_inc;
    }
}

pub fn shutdown() {
    unsafe {
        // Clear the memory in the COLOR_BUFFER
        let mut buffer_option = COLOR_BUFFER.lock().unwrap();
        *buffer_option = None;

        // Destroys the SDL construct to prevent memory leaks
        SDL_DestroyRenderer(SDL_RENDERER);
        SDL_DestroyWindow(SDL_WINDOW);
        SDL_Quit();
    }
}
