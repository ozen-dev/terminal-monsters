use crate::models::dex::{get_dex_mon_by_id, Family};
use crate::models::party::PartyMon;
use ratatui::{prelude::*, widgets::*};

pub fn render_party_ui(
    frame: &mut Frame,
    area: Rect,
    party: &[PartyMon],
    selected_row: usize,
    scroll_position: &mut usize,
) {
    // Define table headers
    let headers = Row::new(vec![
        Cell::from("#"),
        Cell::from("Name"),
        Cell::from("Family"),
        Cell::from("Level"),
        Cell::from("Exp."),
    ])
    .style(Style::default().fg(Color::White));

    // Calculate the visible rows based on the scroll state
    let visible_rows = area.height as usize - 3; // (adjusted for header and borders)
    let start = (*scroll_position).min(party.len().saturating_sub(visible_rows));
    let end = (start + visible_rows).min(party.len());

    // Define table rows
    let rows: Vec<Row> = party[start..end]
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let dex_mon = get_dex_mon_by_id(g.dex_id).unwrap();
            let style = if i + start == selected_row {
                Style::default().bg(match dex_mon.family {
                    Family::Scripting => Color::DarkGray,
                    Family::Web => Color::Red,
                    Family::Mobile => Color::Green,
                    Family::Gaming => Color::Blue,
                    Family::Database => Color::Yellow,
                    Family::Systems => Color::Cyan,
                    Family::Neural => Color::Magenta,
                    Family::Ancient => Color::Rgb(150, 75, 0),
                })
            } else {
                Style::default().fg(match dex_mon.family {
                    Family::Scripting => Color::DarkGray,
                    Family::Web => Color::Red,
                    Family::Mobile => Color::Green,
                    Family::Gaming => Color::Blue,
                    Family::Database => Color::Yellow,
                    Family::Systems => Color::Cyan,
                    Family::Neural => Color::Magenta,
                    Family::Ancient => Color::Rgb(150, 75, 0),
                })
            };
            Row::new(vec![
                Cell::from(format!("{}", dex_mon.id)),
                Cell::from(format!("{}", dex_mon.name)),
                Cell::from(format!(
                    "{}",
                    serde_json::to_string(&dex_mon.family)
                        .unwrap()
                        .trim_matches('"')
                        .to_string()
                )),
                Cell::from(format!("{}", g.level)),
                Cell::from(format!("{}/{}", g.experience_range.0, g.experience_range.1)),
            ])
            .style(style)
        })
        .collect();

    // Define column constraints
    let column_constraints = [
        Constraint::Min(5),
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

    // Render the table
    frame.render_widget(table, area);
}
