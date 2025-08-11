use std::ffi::c_void;
use std::ptr::null;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use sdl2_sys::SDL_CreateTexture;
use sdl2_sys::SDL_DestroyRenderer;
use sdl2_sys::SDL_DestroyWindow;
use sdl2_sys::SDL_DisplayMode;
use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_EventType;
use sdl2_sys::SDL_GetCurrentDisplayMode;
use sdl2_sys::SDL_KeyCode;
use sdl2_sys::SDL_PixelFormatEnum;
use sdl2_sys::SDL_PollEvent;
use sdl2_sys::SDL_Quit;
use sdl2_sys::SDL_RenderClear;
use sdl2_sys::SDL_RenderCopy;
use sdl2_sys::SDL_RenderPresent;
use sdl2_sys::SDL_SetRenderDrawColor;
use sdl2_sys::SDL_SetWindowFullscreen;
use sdl2_sys::SDL_Texture;
use sdl2_sys::SDL_TextureAccess;
use sdl2_sys::SDL_UpdateTexture;
use sdl2_sys::SDL_WindowFlags;
use sdl2_sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_GetError, SDL_INIT_EVERYTHING, SDL_Renderer,
    SDL_WINDOWPOS_CENTERED_MASK, SDL_Window,
};

extern crate sdl2_sys;

static mut WINDOW_WIDTH: i32 = 800;
static mut WINDOW_HEIGHT: i32 = 600;

// unsafe code global variables
static mut SDL_WINDOW: *mut SDL_Window = std::ptr::null_mut();
static mut SDL_RENDERER: *mut SDL_Renderer = std::ptr::null_mut();
static mut SDL_TEXTURE: *mut SDL_Texture = std::ptr::null_mut();

static IS_RUNNING: AtomicBool = AtomicBool::new(false);
static COLOR_BUFFER: Mutex<Option<Vec<u32>>> = Mutex::new(None);

fn initialise_window() -> bool {
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
        SDL_WINDOW = SDL_CreateWindow(
            std::ptr::null(),
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32,
        );

        // Checks if the window was initialised
        assert!(
            !SDL_WINDOW.is_null(),
            "SDL_CreateWindow failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        SDL_RENDERER = SDL_CreateRenderer(SDL_WINDOW, -1, 0);

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

fn clear_color_buffer(color: u32) {
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

fn render_color_buffer() {
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
fn draw_grid() {
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

fn draw_rect(x_pos: u32, y_pos: u32, width: u32, height: u32, color: u32) {
    let mut buffer_option = COLOR_BUFFER
        .lock()
        .expect("Failed to acquire lock for the COLOR_BUFFER");

    unsafe {
        if let Some(buffer) = buffer_option.as_mut() {
            for y in 0..height {
                for x in 0..width {
                    let current_x = x_pos + x;
                    let current_y = y_pos + y;
                    buffer[((WINDOW_WIDTH as u32 * current_y) + current_x) as usize] = color;
                }
            }
        }
    }
}

fn update() {}

fn render() {
    unsafe {
        SDL_SetRenderDrawColor(SDL_RENDERER, 255, 0, 0, 255);
        SDL_RenderClear(SDL_RENDERER);
        clear_color_buffer(0xFF000000);

        draw_grid();
        draw_rect(200, 200, 120, 160, 0xFF00FF00);

        render_color_buffer();
        SDL_RenderPresent(SDL_RENDERER);
    }
}

fn shutdown() {
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
