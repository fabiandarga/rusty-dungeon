use tui::layout::Constraint;
use tui::layout::Layout;
use tui::style::{Style,Color};
use tui::widgets::{Block, Borders, BorderType};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::widgets::Widget;
use tui::layout::Alignment;
use tui::text::Text;
use tui::text::Span;

#[derive(Debug, Clone)]
pub struct MessageBlock<'a> {
    title: Text<'a>,
    /// The text to display
    text: Span<'a>,
}

impl<'a> MessageBlock<'a> {
    pub fn new<T>(title: T, text: Span<'a>) -> MessageBlock<'a>
    where
        T: Into<Text<'a>>,
    {
        MessageBlock {
            title: title.into(),
            text,
        }
    }
}

impl<'a> Widget for MessageBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Items")
            .border_type(BorderType::Plain);

        block.render(area, buf);

        let message = Paragraph::new(self.text)
            .alignment(Alignment::Center);

        let chunks = Layout::default()
            .constraints([Constraint::Min(1)])
            .margin(2)
            .split(area);

        message.render(chunks[0], buf);
    }
}
