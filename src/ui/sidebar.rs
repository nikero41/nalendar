use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Paragraph, Widget},
};

pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &Sidebar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().border_type(BorderType::Rounded);

        let layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).split(block.inner(area));
        block.render(area, buf);

        Paragraph::new("Calendars").render(layout[0], buf);
        Paragraph::new("Calendars").render(layout[1], buf);
    }
}
