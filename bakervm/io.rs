use core::Config;
use core::Event;
use core::error::*;
use core::typedef::*;
use sdl2;
use sdl2::event::Event as SDL2Event;
use sdl2::event::EventType as SDL2EventType;
use sdl2::pixels::Color;
use std::sync::{Arc, Barrier};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn start(
    frame_receiver: Receiver<Frame>, event_sender: Sender<Event>, config: Config,
    barrier: Arc<Barrier>
) -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let hide_cursor = config.display.hide_cursor;


    sdl_context.mouse().show_cursor(!hide_cursor);

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
    event_pump.disable_event(SDL2EventType::First);
    event_pump.disable_event(SDL2EventType::AppTerminating);
    event_pump.disable_event(SDL2EventType::AppLowMemory);
    event_pump.disable_event(SDL2EventType::AppWillEnterBackground);
    event_pump.disable_event(SDL2EventType::AppDidEnterBackground);
    event_pump.disable_event(SDL2EventType::AppWillEnterForeground);
    event_pump.disable_event(SDL2EventType::AppDidEnterForeground);
    event_pump.disable_event(SDL2EventType::Window);
    event_pump.disable_event(SDL2EventType::TextEditing);
    event_pump.disable_event(SDL2EventType::TextInput);
    event_pump.disable_event(SDL2EventType::MouseWheel);
    event_pump.disable_event(SDL2EventType::JoyAxisMotion);
    event_pump.disable_event(SDL2EventType::JoyBallMotion);
    event_pump.disable_event(SDL2EventType::JoyHatMotion);
    event_pump.disable_event(SDL2EventType::JoyButtonDown);
    event_pump.disable_event(SDL2EventType::JoyButtonUp);
    event_pump.disable_event(SDL2EventType::JoyDeviceAdded);
    event_pump.disable_event(SDL2EventType::JoyDeviceRemoved);
    event_pump.disable_event(SDL2EventType::ControllerAxisMotion);
    event_pump.disable_event(SDL2EventType::ControllerButtonDown);
    event_pump.disable_event(SDL2EventType::ControllerButtonUp);
    event_pump.disable_event(SDL2EventType::ControllerDeviceAdded);
    event_pump.disable_event(SDL2EventType::ControllerDeviceRemoved);
    event_pump.disable_event(SDL2EventType::ControllerDeviceRemapped);
    event_pump.disable_event(SDL2EventType::FingerDown);
    event_pump.disable_event(SDL2EventType::FingerUp);
    event_pump.disable_event(SDL2EventType::FingerMotion);
    event_pump.disable_event(SDL2EventType::DollarGesture);
    event_pump.disable_event(SDL2EventType::DollarRecord);
    event_pump.disable_event(SDL2EventType::MultiGesture);
    event_pump.disable_event(SDL2EventType::ClipboardUpdate);
    event_pump.disable_event(SDL2EventType::DropFile);
    event_pump.disable_event(SDL2EventType::User);
    event_pump.disable_event(SDL2EventType::Last);

    barrier.wait();

    let mut last_event = None;

    'main: loop {
        let new_event = event_pump.poll_event();
        if new_event != last_event {
            // get the inputs here
            if let Some(event) = new_event.clone() {
                match event {
                    SDL2Event::Quit { .. } => {
                        event_sender.send(Event::Halt).chain_err(|| "unable to send event")?;

                        break 'main;
                    }
                    _ => {}
                }

                if config.input_enabled {
                    match event {
                        SDL2Event::KeyDown { keycode: Some(key), .. } => {
                            let res = event_sender.send(Event::KeyDown(key as Address));

                            if let Err(..) = res {
                                break 'main;
                            }
                        }
                        SDL2Event::KeyUp { keycode: Some(key), .. } => {
                            let res = event_sender.send(Event::KeyUp(key as Address));

                            if let Err(..) = res {
                                break 'main;
                            }
                        }
                        SDL2Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                            let res = event_sender.send(
                                Event::MouseDown {
                                    x: (x as Float / config.display.default_scale).floor() as
                                       Address,
                                    y: (y as Float / config.display.default_scale).floor() as
                                       Address,
                                    button: mouse_btn as Address,
                                },
                            );

                            if let Err(..) = res {
                                break 'main;
                            }
                        }
                        SDL2Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                            let res = event_sender.send(
                                Event::MouseUp {
                                    x: (x as Float / config.display.default_scale).floor() as
                                       Address,
                                    y: (y as Float / config.display.default_scale).floor() as
                                       Address,
                                    button: mouse_btn as Address,
                                },
                            );

                            if let Err(..) = res {
                                break 'main;
                            }
                        }
                        SDL2Event::MouseMotion { x, y, .. } => {
                            let res = event_sender.send(
                                Event::MouseMove {
                                    x: (x as Float / config.display.default_scale).floor() as
                                       Address,
                                    y: (y as Float / config.display.default_scale).floor() as
                                       Address,
                                },
                            );

                            if let Err(..) = res {
                                break 'main;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if new_event.is_some() {
            last_event = new_event;
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
    }

    Ok(())
}
