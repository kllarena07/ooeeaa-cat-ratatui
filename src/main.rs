use ratatui::{DefaultTerminal, Frame, text::Text};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};
use std::{io, thread, time::Duration};

fn import_all_images() -> Vec<StatefulProtocol> {
    let mut image_protocol_collection: Vec<StatefulProtocol> = vec![];
    for count in 0..=102 {
        let picker = Picker::from_fontsize((8, 12));
        let path = format!("./images/frame_{}.png", count);
        let dyn_img = image::ImageReader::open(path).unwrap().decode().unwrap();
        let image = picker.new_resize_protocol(dyn_img);
        image_protocol_collection.push(image);
    }
    image_protocol_collection
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let collection = import_all_images();

    let mut app = App {
        running: true,
        collection,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    running: bool,
    collection: Vec<StatefulProtocol>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            let frame_duration = Duration::from_millis(1000 / 30); // 30 FPS

            for count in 0..=102 {
                terminal.draw(|frame| self.draw(frame, count))?;
                thread::sleep(frame_duration);
            }
        }

        let _ = self
            .collection
            .iter_mut()
            .map(|image| image.last_encoding_result());
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, count: i32) {
        frame.render_widget(Text::from(count.to_string()), frame.area());
        let image = StatefulImage::default();
        let selected_frame = self.collection.get_mut(count as usize).unwrap();
        frame.render_stateful_widget(image, frame.area(), selected_frame);
        // EXPECTED: &mut _
    }
}
