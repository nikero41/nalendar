use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct MonthView;

impl MonthView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &MonthView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let month_layout = Layout::vertical([
            Constraint::Max(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let week_layout = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]);

        header_view(month_layout[0], buf);

        (1..=31).for_each(|day| {
            let week = (day / 7) + 1;

            let week_area = month_layout[week];
            let day_area = week_layout.split(week_area)[day % 7];
            Paragraph::new(day.to_string())
                .block(Block::bordered())
                .render(day_area, buf);
        });
    }
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn header_view(area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
    let week_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .split(area);

    (1..=7).for_each(|day| {
        let day_area = week_layout[day % 7];
        Paragraph::new("Monday")
            .block(Block::new().borders(Borders::BOTTOM))
            .centered()
            .render(day_area, buf);
    })
}
