//! Example 'direct-cursor'
//!
//! Explore cursor functions in direct mode
//!

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let ncd = NcDirect::new()?;

    let cols = ncd.dim_x();
    let rows = ncd.dim_y();
    println!("terminal size (rows, cols): {}, {}", rows, cols);

    ncd.putstr(0, "The current coordinates are")?;

    for _n in 0..40 {
        fsleep![ncd, 0, 30];
        ncd.putstr(0, ".")?;
    }

    let (cy, cx) = ncd.cursor_yx()?;
    ncd.putstr(0, &format!(" ({},{})\n", cy, cx))?;
    sleep![1];

    let sentence = vec!["And", "now", "I", "will", "clear", "the", "screen", ".", ".", "."];
    for word in sentence {
        ncd.putstr(0, &format!["{} ", word])?;
        fsleep![ncd, 0, 150];
    }
    sleep![0, 300];
    ncd.putstr(0, "\nbye!\n\n")?;
    fsleep![ncd, 0, 600];

    ncd.clear()?;
    sleep![1];

    ncd.stop()?;
    Ok(())
}
