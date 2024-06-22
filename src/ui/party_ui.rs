use crate::models::dex::Type;
use crate::models::party::PartyMon;
use ratatui::{prelude::*, widgets::*};

pub fn render_party_ui(frame: &mut Frame, area: Rect, party: &[PartyMon], selected_row: usize) {
    // Define table headers
    let headers = Row::new(vec![
        Cell::from("Name"),
        Cell::from("Type"),
        Cell::from("Level"),
        Cell::from("Exp."),
    ])
    .style(Style::default().fg(Color::White).bg(Color::Black));

    // Define table rows
    let rows: Vec<Row> = party
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let style = if i == selected_row {
                Style::default().bg(match g.dex_entry.types[0] {
                    Type::Origin => Color::Rgb(140, 140, 140),
                    Type::Fire => Color::Red,
                    Type::Water => Color::Blue,
                    Type::Grass => Color::Green,
                    Type::Electric => Color::Yellow,
                    Type::Sonic => Color::Cyan,
                    Type::Dark => Color::Rgb(48, 48, 48),
                    Type::Garbage => Color::Rgb(110, 62, 0),
                    Type::Mystic => Color::Magenta,
                    Type::Legendary => Color::Rgb(204, 102, 0),
                })
            } else {
                Style::default().bg(Color::Black)
            };
            Row::new(vec![
                Cell::from(format!("{}", g.dex_entry.name)),
                Cell::from(format!(
                    "{}",
                    g.dex_entry
                        .types
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )),
                Cell::from(format!("{}", g.level)),
                Cell::from(format!("{}/{}", g.experience_range.0, g.experience_range.1)),
            ])
            .style(style)
        })
        .collect();

    // Define column constraints
    let column_constraints = [
        Constraint::Percentage(100),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    // Create the table
    let table = Table::new(rows, column_constraints)
        .header(headers)
        .block(
            Block::new()
                .title("Party")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(table, area);
}
