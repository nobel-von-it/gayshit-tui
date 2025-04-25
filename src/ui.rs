use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget},
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn create_constraints_per(size: usize) -> Vec<Constraint> {
    let mut constraints = Vec::with_capacity(size);
    for _ in 0..size {
        constraints.push(Constraint::Percentage(100 / size as u16));
    }
    constraints
}

fn create_constraints_len(size: usize, len: u16) -> Vec<Constraint> {
    let mut constraints = Vec::with_capacity(size);
    for _ in 0..size {
        constraints.push(Constraint::Length(len));
    }
    constraints
}

fn create_choice(text: &str, selected: bool) -> Paragraph {
    Paragraph::new(text)
        .style(if selected {
            Style::default().add_modifier(ratatui::style::Modifier::BOLD)
        } else {
            Style::default()
        })
        .block(Block::bordered())
        .centered()
}

// fn render(self, area: Rect, buf: &mut Buffer)
pub fn render_main_menu(choices: &[String], selected: usize, area: Rect, buf: &mut Buffer) {
    let menu_block = Block::bordered().title("Menu");

    let window_layout = centered_rect(90, 90, area);
    let subwindow_layout = centered_rect(95, 95, window_layout);

    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(subwindow_layout);
    Paragraph::new("")
        .block(menu_block)
        .render(window_layout, buf);

    let choices_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(create_constraints_len(choices.len(), 3))
        .split(game_layout[0]);
    for (i, choice) in choices.iter().enumerate() {
        let text = &choice;
        create_choice(text, i == selected).render(choices_layout[i], buf);
    }
}
