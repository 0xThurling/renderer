macro_rules! sdl_ticks_passed {
    ($a: expr, $b: expr) => {
        // $b is current time, $a ic ticks passed
        (($b).wrapping_sub($a) as i32) <= 0
    };
}
