use super::App;
use ratatui::prelude::{Direction, Stylize};
use ratatui::text::Span;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, List, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let frame_rect = frame.size();
    if frame_rect.height <= 5 || frame_rect.width <= 45 {
        frame.render_widget(Paragraph::new("Window is too small!"), frame_rect);
        return;
    }
    let style: Style = Style::default().fg(Color::LightCyan).bg(Color::Black);

    let areas = calculate_areas(frame_rect);
    // Render help
    frame.render_widget(
        help_paragraph().block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("[ Keys ]")
                .title_alignment(Alignment::Center)
                .border_style(style),
        ),
        areas.help,
    );
    // Render history
    let history = app.history();
    let offset = history.len() as isize - (areas.history.height - 2) as isize;
    frame.render_widget(
        List::new(
            history
                .iter()
                .skip(if offset > 0 { offset as usize } else { 0 })
                .map(|s| Line::from(s.to_string()).alignment(Alignment::Right)),
        )
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("[ History ]")
                    .title_alignment(Alignment::Center)
                    .border_style(style),
            ),

        areas.history,
    );
    // Render the accumulator value
    frame.render_widget(
        Paragraph::new(app.acc().to_string())
            .right_aligned()
            .style(style),
        areas.accumulator,
    );
    // Render the current operator and input
    frame.render_widget(
        Paragraph::new(format!("{} {}", app.current_operation(), app.digits())).right_aligned(),
        areas.input,
    );
}

fn help_paragraph<'a>() -> Paragraph<'a> {
    let style: Style = Style::default()
        .fg(Color::LightCyan)
        .bg(Color::Black)
        .bold();

    fn line<'a>(key: &'a str, style: Style, desc: &'a str) -> Line<'a> {
        let key = format!(" {key}");
        let desc = format!(" {}", desc);
        Line::from(vec![Span::styled(key, style), Span::raw(desc)])
    }
    let help_text = vec![
        line("0-9", style, "Number"),
        line(".", style, "Decimal"),
        line("Esc", style, "Clear"),
        line("Backspace", style, ""),
        line("+", style, "Add"),
        line("-", style, "Subtract"),
        line("*", style, "Multiply"),
        line("/", style, "Divide"),
        line("=", style, "Equal"),
        line("n", style, "Make input negative"),
        line("p", style,"Make input negative"),
        line("Ctrl+Q", style,"Exit"),
    ];
    Paragraph::new(help_text)
}

struct Areas {
    history: Rect,
    help: Rect,
    accumulator: Rect,
    input: Rect,
}
fn calculate_areas(area: Rect) -> Areas {
    let rows = Layout::default()
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);
    let top_columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Min(1)])
        .split(rows[0]);
    let left_rows = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(14)])
        .split(top_columns[0]);
    Areas {
        history: top_columns[1],
        help: left_rows[1],
        accumulator: rows[1],
        input: rows[2],
    }
}
