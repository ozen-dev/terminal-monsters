use ratatui::{prelude::*, widgets::*};

pub fn render_header_ui(frame: &mut Frame, area: Rect) {
    let footer_text = " (↑) up | (↓) down | (↵) select | (esc) back | (w) wiki | (q) quit";
    let footer = Paragraph::new(footer_text)
        .block(
            Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Terminal Monsters Inc."),
        )
        .alignment(Alignment::Left);
        
    frame.render_widget(footer, area);
}
