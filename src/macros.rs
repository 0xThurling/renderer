macro_rules! sdl_ticks_passed {
    ($a: expr, $b: expr) => {
        // $b is my deadline, $a is my current ticks 
        (($b).wrapping_sub($a) as i32) <= 0
    };
}


