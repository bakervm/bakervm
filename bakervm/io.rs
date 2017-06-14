use definitions::Config;
use definitions::Event;
use definitions::error::*;
use definitions::typedef::*;
use sdl2;
use sdl2::event::Event as SDL2Event;
use sdl2::pixels::Color;
use std::sync::{Arc, Barrier};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

pub fn start(
    frame_receiver: Receiver<Frame>, event_sender: Sender<Event>, config: Config,
    barrier: Arc<Barrier>
) -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            config.title.as_ref(),
            (config.display.default_scale * (config.display.resolution.width as Float)).round() as
            u32,
            (config.display.default_scale * (config.display.resolution.height as Float))
                .round() as u32,
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

    canvas
        .set_scale(
            config.display.default_scale as f32,
            config.display.default_scale as f32,
        )?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_count = 0;

    barrier.wait();

    let mut now_before = Instant::now();

    'main: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                SDL2Event::Quit { .. } => {
                    event_sender.send(Event::Halt).chain_err(|| "unable to send event")?;

                    break 'main;
                }
                SDL2Event::KeyDown { keycode: Some(key), .. } => {
                    event_sender
                        .send(Event::KeyDown(key as Address))
                        .chain_err(|| "unable to send event")?;
                }
                SDL2Event::KeyUp { keycode: Some(key), .. } => {
                    event_sender
                        .send(Event::KeyUp(key as Address))
                        .chain_err(|| "unable to send event")?;
                }
                SDL2Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    event_sender
                        .send(
                            Event::MouseDown {
                                x: (x as Float / config.display.default_scale).floor() as Address,
                                y: (y as Float / config.display.default_scale).floor() as Address,
                                button: mouse_btn as Address,
                            },
                        )
                        .chain_err(|| "unable to send event")?;
                }
                SDL2Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    event_sender
                        .send(
                            Event::MouseUp {
                                x: (x as Float / config.display.default_scale).floor() as Address,
                                y: (y as Float / config.display.default_scale).floor() as Address,
                                button: mouse_btn as Address,
                            },
                        )
                        .chain_err(|| "unable to send event")?;
                }
                _ => {}
            }
        }


        // Receive a frame
        let maybe_frame = frame_receiver.try_recv();
        if let Ok(frame) = maybe_frame {

            let mut index = 0;
            for y_coord in 0..config.display.resolution.height {
                for x_coord in 0..config.display.resolution.width {
                    if let Some(&(r, g, b)) = frame.get(index) {
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
        } else {
            thread::sleep(Duration::from_millis(10));
        }

        let secs_elapsed = now_before.elapsed();

        if secs_elapsed >= Duration::from_millis(1000) {
            println!("FPS: {:?}", frame_count / secs_elapsed.as_secs());
            now_before = Instant::now();
            frame_count = 0;
        }

        frame_count += 1;
    }

    Ok(())
}
