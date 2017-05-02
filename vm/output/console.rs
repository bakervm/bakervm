use output::Mountable;
use std::sync::mpsc::{self, Sender};
use std::thread;

/// A regular console output
pub struct Console {}

impl Mountable for Console {
    type DataFormat = Vec<char>;

    fn run(&self) -> Sender<Self::DataFormat> {
        let (sender, receiver) = mpsc::channel::<Self::DataFormat>();

        thread::spawn(
            move || {
                let receiver = receiver;

                'console: loop {
                    if let Ok(data) = receiver.recv() {
                        let recv_string: String = data.into_iter().collect();

                        print!("{}", recv_string);
                    } else {
                        break 'console;
                    }
                }
            },
        );

        sender
    }
}
