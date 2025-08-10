use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

mod constants;
use constants::*;

use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_PollEvent;
use sdl2_sys::SDL_RenderClear;
use sdl2_sys::SDL_RenderPresent;
use sdl2_sys::SDL_SetRenderDrawColor;
use sdl2_sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_GetError, SDL_INIT_EVERYTHING, SDL_Renderer,
    SDL_WINDOWPOS_CENTERED_MASK, SDL_Window,
};

extern crate sdl2_sys;

// unsafe code global variables
static mut SDL_WINDOW: *mut SDL_Window = std::ptr::null_mut();
static mut SDL_RENDERER: *mut SDL_Renderer = std::ptr::null_mut();

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

fn initialise_window() -> bool {
    unsafe {
        let result = sdl2_sys::SDL_Init(SDL_INIT_EVERYTHING);

        // Checks if the initialisation failed
        assert!(
            result == 0,
            "SDL init failed: {:?}",
            std::ffi::CStr::from_ptr(SDL_GetError())
        );

        // Create SDL WINDOW
        SDL_WINDOW = SDL_CreateWindow(
            std::ptr::null(),
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            SDL_WINDOWPOS_CENTERED_MASK as i32,
            800,
            600,
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
    }

    true
}

fn setup() {}

fn process_input() {
    unsafe {
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

fn update() {}

fn render() {
    unsafe {
        SDL_SetRenderDrawColor(SDL_RENDERER, 255, 0, 0, 255);
        SDL_RenderClear(SDL_RENDERER);

        // ...
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
}
