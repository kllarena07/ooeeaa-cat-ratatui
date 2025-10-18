use ansi_to_tui::IntoText as _;
use crossterm::event::KeyCode;
use rascii_art::{RenderOptions, charsets, render_to};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
};
use std::{io, sync::mpsc, thread, time::Duration};

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
        count: 0,
    };

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_counter_events = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_to_counter_events);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Counter(usize),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    let framerate = 60;
    let frame_duration = Duration::from_millis(1000 / framerate);

    loop {
        for count in 0..=102 {
            tx.send(Event::Counter(count)).unwrap();
            thread::sleep(frame_duration);
        }
    }
}

struct App<'a> {
    running: bool,
    paragraph_collection: Vec<Paragraph<'a>>,
    count: usize,
}

impl<'a> App<'a> {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while self.running {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Counter(count) => self.count = count,
            }

            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let display_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Length(99)])
            .split(frame.area());
        frame.render_widget(
            Paragraph::new(self.count.to_string()).block(Block::new()),
            display_area[0],
        );

        let paragraph = &self.paragraph_collection[self.count];

        frame.render_widget(paragraph, display_area[1]);
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
