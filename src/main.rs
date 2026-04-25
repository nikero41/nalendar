use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    widgets::{Paragraph, Widget},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let _ = terminal.draw(|frame| frame.render_widget::<&App>(self, frame.area()));
            self.handle_event()?;
        }
        Ok(())
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.exit = true;
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Hello world!").render(area, buf);
    }
}
