use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Margin, Rect},
    style::Color,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Line},
    },
};

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

enum AppEvent {
    Tick,
    Key(KeyCode),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    // app.key_pressed = 'B';

    let (tx, rx) = channel();

    // Timer thread
    let tx_tick = tx.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
            if tx_tick.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    // Input thread
    thread::spawn(move || {
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if tx.send(AppEvent::Key(key.code)).is_err() {
                    break;
                }
            }
        }
    });

    loop {
        terminal.draw(|frame| app.draw(frame))?;

        match rx.recv()? {
            AppEvent::Tick => app.tick(),
            AppEvent::Key(KeyCode::Char('q')) => break,
            // AppEvent::Key(other) => app.last_key = Some(other),
            _ => {}
        }
    }

    ratatui::restore();
    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    y_positions: [u16; 10],
    letters_row: [char; 10],
    inner_height: u16,
    key_pressed: char,
}

impl App {
    pub fn new() -> Self {
        App {
            y_positions: [0; 10],
            letters_row: [' '; 10],
            inner_height: 12,
            key_pressed: ' ',
        }
    }

    fn tick(&mut self) {
        // Random add a letter

        for (idx, x) in self.y_positions.iter_mut().enumerate() {
            if self.letters_row[idx] != ' ' {
                *x += 1;
                if *x >= self.inner_height - 2 {
                    *x = 0;
                    self.letters_row[idx] = ' ';
                }
            }
            self.key_pressed = ' ';
        }

        self.letters_row[0] = 'A';
    }

    fn draw(&self, frame: &mut Frame) {
        let area = Rect {
            x: 10,
            y: 5,
            width: 42,
            height: self.inner_height,
        };

        // Outer box
        let block = Block::default().borders(Borders::ALL);
        frame.render_widget(block, area);

        let inner = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        // Horizontal line spanning full width + 2 (over the borders)
        let line_y = inner.y + (inner.height as f64 / 1.2) as u16;
        let line = "─".repeat((area.width) as usize);
        let line_area = Rect {
            x: area.x,
            y: line_y,
            width: area.width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(line), line_area);

        for x in 0..10 {
            let letter = self.letters_row[x].to_string();
            if letter != " " {
                let letter_area = Rect {
                    x: inner.x + 0,
                    y: inner.y + self.y_positions[x],
                    width: 1,
                    height: 1,
                };
                frame.render_widget(Paragraph::new(letter), letter_area);
            }
        }
        if self.key_pressed != ' ' {
            let pressed_area = Rect {
                x: inner.x,
                y: inner.y,
                width: 1,
                height: 1,
            };
            frame.render_widget(Paragraph::new(self.key_pressed.to_string()), pressed_area);
        }
    }
}
