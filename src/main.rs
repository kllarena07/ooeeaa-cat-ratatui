// use std::{thread, time::Duration};

// fn main() {
//     let frame_duration = Duration::from_millis(1000 / 30); // 30 FPS

//     for count in 0..=103 {
//         println!("{}", count);
//         thread::sleep(frame_duration);
//     }
// }

use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let picker = Picker::from_fontsize((8, 12));
    let dyn_img = image::ImageReader::open("./images/frame_0.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = picker.new_resize_protocol(dyn_img);

    let mut app = App {
        running: true,
        image,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    running: bool,
    image: StatefulProtocol,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        self.image.last_encoding_result();
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let image = StatefulImage::default();
        frame.render_stateful_widget(image, frame.area(), &mut self.image);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            _ => {}
        }

        Ok(())
    }
}
