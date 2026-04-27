use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::ui::{month_view::MonthView, sidebar::Sidebar};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
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
        let layout = Layout::horizontal([Constraint::Max(25), Constraint::Fill(1)]).split(area);

        Sidebar::new().render(layout[0], buf);
        MonthView::new().render(layout[1], buf);
    }
}
