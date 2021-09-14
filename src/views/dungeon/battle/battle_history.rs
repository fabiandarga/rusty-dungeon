use tui::text::Span;
use tui::{
    buffer::Buffer,
    layout::Rect,
    layout::Alignment,
    widgets::{ Widget, Paragraph, Block, Borders, BorderType },
    style::{ Color, Style }
};

use tui::text::Spans;

use crate::models::BattleEvent;
use crate::models::BattleEvents;

pub struct BattleHistory {
    history: BattleEvents,
}

impl BattleHistory {
    pub fn new(history: BattleEvents) -> BattleHistory {
        BattleHistory { history }
    }

    fn get_entry(&self, event: &BattleEvent) -> Spans {
        Spans::from(vec![
            Span::raw(event.title.to_owned())
        ])
    }

    fn get_text_content(&self) -> Vec<Spans> {
        let content: Vec<Spans> = self.history.events.iter().map(|event| self.get_entry(event)).collect();
        content
    }
}

impl Widget for BattleHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {


        let message = Paragraph::new(self.get_text_content())
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Battle Log")
                .border_type(BorderType::Plain)
            );

        message.render(area, buf);
    }
}