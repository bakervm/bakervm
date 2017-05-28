use definitions::config::VMConfig;
use definitions::program::Interrupt;
use definitions::typedef::*;
use error::*;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub fn start(frame_receiver: Receiver<Frame>, interrupt_sender: Sender<Interrupt>, config: VMConfig)
    -> VMResult<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            config.title.as_ref(),
            (config.display.scale * (config.display.resolution.width as Float)).round() as u32,
            (config.display.scale * (config.display.resolution.height as Float)).round() as u32,
        )
        .position_centered()
        .build()
        .chain_err(|| "unable to build window")?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .chain_err(|| "unable to convert window into canvas")?;

    canvas.set_scale(config.display.scale as f32, config.display.scale as f32)?;

    let mut event_pump = sdl_context.event_pump()?;

    'main: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    interrupt_sender
                        .send(
                            Interrupt {
                                signal_id: 0,
                                args: Vec::new(),
                            },
                        )
                        .chain_err(|| "unable to send interrupt")?;

                    break 'main;
                }
                _ => {
                    // TODO: Send interrupt here
                }
            }
        }

        // Receive a frame
        let maybe_frame = frame_receiver.try_recv();
        if let Ok(frame) = maybe_frame {

            let mut index = 0;
            for y_coord in 0..config.display.resolution.height {
                for x_coord in 0..config.display.resolution.width {
                    if let Some(raw_color) = frame.get(index) {
                        let r: u8 = (raw_color >> 16) as u8;
                        let g: u8 = (raw_color >> 8) as u8;
                        let b: u8 = *raw_color as u8;

                        canvas.set_draw_color(Color::RGB(r, g, b));

                        canvas.draw_point((x_coord as i32, y_coord as i32))?;
                        index += 1;
                    } else {
                        bail!("no color point available at index {}", index);
                    }
                }
            }

            canvas.present();
        } else if let Err(TryRecvError::Disconnected) = maybe_frame {
            break 'main;
        }
    }

    Ok(())
}
