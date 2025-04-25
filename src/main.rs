use gayshit_tui::{app::Game, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut game = Game::default();
    game.load_menu_by_state()?;

    let res = run(&mut terminal, &mut game);

    ratatui::restore();

    res
}

fn run(terminal: &mut DefaultTerminal, game: &mut Game) -> Result<()> {
    while !game.should_quit {
        terminal.draw(|f| game.draw(f))?;
        game.handle_event()?;
    }
    Ok(())
}

fn render(f: &mut Frame) {
    f.render_widget("hello world", f.area());
}
