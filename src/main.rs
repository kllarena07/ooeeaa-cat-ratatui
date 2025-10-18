use ansi_to_tui::IntoText as _;
use crossterm::event::KeyCode;
use rascii_art::{RenderOptions, charsets, render_to};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
};
use std::{io, thread, time::Duration};

fn construct_all_paragraphs<'a>() -> Vec<Paragraph<'a>> {
    let mut paragraph_collection: Vec<Paragraph> = vec![];

    for count in 0..=102 {
        let mut buffer = String::new();
        let path = format!("./images/frame_{}.png", count);

        render_to(
            path,
            &mut buffer,
            &RenderOptions::new()
                .width(65)
                .colored(true)
                .charset(charsets::BLOCK),
        )
        .unwrap();

        let output = buffer.into_text().unwrap();
        let output_as_paragraph = Paragraph::new(output);

        paragraph_collection.push(output_as_paragraph);

        println!("Loaded frame {}/102", count);
    }

    paragraph_collection
}

fn main() -> io::Result<()> {
    let paragraph_collection = construct_all_paragraphs();
    let mut terminal = ratatui::init();

    let mut app = App {
        running: true,
        paragraph_collection,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

struct App<'a> {
    running: bool,
    paragraph_collection: Vec<Paragraph<'a>>,
}

impl<'a> App<'a> {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            // match crossterm::event::read()? {
            //     crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
            //     _ => {}
            // }
            let frame_duration = Duration::from_millis(1000 / 60); // 60 FPS

            for count in 0..=102 {
                terminal.draw(|frame| self.draw(frame, count))?;
                thread::sleep(frame_duration);
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, count: u32) {
        let display_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Length(99)])
            .split(frame.area());
        frame.render_widget(
            Paragraph::new(count.to_string()).block(Block::new()),
            display_area[0],
        );

        let count_usize = count as usize;
        let paragraph = &self.paragraph_collection[count_usize];

        frame.render_widget(paragraph, display_area[1]);
    }

    // fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
    //     match key_event.code {
    //         KeyCode::Char('q') => {
    //             self.running = false;
    //         }
    //         _ => {}
    //     }

    //     Ok(())
    // }
}
