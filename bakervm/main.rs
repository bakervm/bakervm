extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate definitions;
// extern crate sdl2;
extern crate ieee754;
extern crate num;

mod vm;
mod error;

use clap::{App, Arg};
use error::*;
// use sdl2::event::Event;
// use sdl2::rect::Rect;
use vm::VM;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> VMResult<()> {
    let matches = App::new("bakerVM")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .about("A virtual machine for building and running retro games")
        .arg(
            Arg::with_name("input")
                .index(1)
                .help("Sets the image file to use. Uses a standard image if nothing is specified.")
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or("").to_string();

    let mut vm = VM::new();

    vm.exec(input).chain_err(|| "unable to exec file")?;

    // // start sdl2 with everything
    // let ctx = sdl2::init().unwrap();
    // let video_ctx = ctx.video().unwrap();
    //
    // // Create a window
    // let window = video_ctx.window("bakerVM", 640, 480)
    //     .position_centered()
    //     .opengl()
    //     .build()
    //     .chain_err(|| "unable to create window")?;
    //
    // // Create a rendering context
    // let mut renderer = window.renderer().build().chain_err(|| "unable to create
    // renderer")?;
    //
    // // Set the drawing color to a light blue.
    // let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));
    //
    // // Clear the buffer, using the light blue color set above.
    // let _ = renderer.clear();
    //
    // // Set the drawing color to a darker blue.
    // let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 153, 204));
    //
    // // Create centered Rect, draw the outline of the Rect in our dark blue color.
    // let border_rect = Rect::new(320 - 64, 240 - 64, 128, 128);
    // let _ = renderer.draw_rect(border_rect);
    //
    // // Create a smaller centered Rect, filling it in the same dark blue.
    // let inner_rect = Rect::new(320 - 60, 240 - 60, 120, 120);
    // let _ = renderer.fill_rect(inner_rect);
    //
    // // Swap our buffer for the present buffer, displaying it.
    // let _ = renderer.present();
    //
    // let mut events = ctx.event_pump().unwrap();
    //
    // // loop until we receive a QuitEvent
    // 'event: loop {
    //     for event in events.poll_iter() {
    //         match event {
    //             Event::Quit { .. } => break 'event,
    //             _ => continue,
    //         }
    //     }
    // }

    Ok(())
}
