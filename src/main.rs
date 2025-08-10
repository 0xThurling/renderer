use std::ffi::c_void;
use std::ptr::null;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

mod constants;
use constants::*;

use sdl2_sys::SDL_CreateTexture;
use sdl2_sys::SDL_DestroyRenderer;
use sdl2_sys::SDL_DestroyWindow;
use sdl2_sys::SDL_DisplayMode;
use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_GetCurrentDisplayMode;
use sdl2_sys::SDL_PollEvent;
use sdl2_sys::SDL_Quit;
use sdl2_sys::SDL_RenderClear;
use sdl2_sys::SDL_RenderCopy;
use sdl2_sys::SDL_RenderPresent;
use sdl2_sys::SDL_SetRenderDrawColor;
use sdl2_sys::SDL_SetWindowFullscreen;
use sdl2_sys::SDL_Texture;
use sdl2_sys::SDL_UpdateTexture;
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
            SDL_WINDOW_BORDERLESS, // SDL_WINDOW_BORDERLESS:
                                   // Needs to be specified like this since I don't have
                                   // access to all constants
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

        let result = SDL_SetWindowFullscreen(SDL_WINDOW, SDL_WINDOW_FULLSCREEN);
        
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
            SDL_PIXELFORMAT_ARGB8888,
            SDL_TEXTUREACCESS_STREAMING,
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
            x if x == SDL_QUIT => {
                IS_RUNNING.store(false, Ordering::SeqCst);
            }
            x if x == SDL_KEYDOWN => {
                if event.key.keysym.sym == SDLK_ESCAPE {
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

fn update() {}

fn render() {
    unsafe {
        SDL_SetRenderDrawColor(SDL_RENDERER, 255, 0, 0, 255);
        SDL_RenderClear(SDL_RENDERER);

        render_color_buffer();

        clear_color_buffer(0xFFFFFF00);

        // ...
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
