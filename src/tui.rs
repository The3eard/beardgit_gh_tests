//! Single-screen TUI dashboard backed by ratatui.
//!
//! Status: **prototype.** Renders open tasks in a table with vim-style
//! keybindings (`j`/`k` to move, `x` to toggle done, `q` to quit). No tag
//! filter or search yet — those land before the branch is ready for review.

use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Row, Table, TableState};
use ratatui::Terminal;

use crate::store::Store;

pub fn run() -> Result<()> {
    let mut store = Store::load_default()?;
    let mut terminal = setup_terminal()?;
    let result = main_loop(&mut terminal, &mut store);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    store: &mut Store,
) -> Result<()> {
    let mut state = TableState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1)])
                .split(area);

            let rows: Vec<Row> = store
                .list(false, None)
                .iter()
                .map(|t| {
                    Row::new(vec![
                        format!("#{}", t.id),
                        t.title.clone(),
                        t.tag.clone().unwrap_or_default(),
                        t.due.map(|d| d.to_string()).unwrap_or_default(),
                    ])
                })
                .collect();

            let widths = [
                Constraint::Length(6),
                Constraint::Percentage(60),
                Constraint::Length(12),
                Constraint::Length(12),
            ];

            let table = Table::new(rows, widths)
                .header(Row::new(vec!["id", "title", "tag", "due"]).style(Style::new().add_modifier(Modifier::BOLD)))
                .block(Block::default().borders(Borders::ALL).title(" tasklog "))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

            frame.render_stateful_widget(table, chunks[0], &mut state.clone());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => move_selection(&mut state, store, 1),
                KeyCode::Char('k') | KeyCode::Up => move_selection(&mut state, store, -1),
                KeyCode::Char('x') => toggle_done(&mut state, store)?,
                _ => {}
            }
        }
    }

    store.save()?;
    Ok(())
}

fn move_selection(state: &mut TableState, store: &Store, delta: isize) {
    let len = store.list(false, None).len();
    if len == 0 {
        state.select(None);
        return;
    }
    let current = state.selected().unwrap_or(0) as isize;
    let next = (current + delta).rem_euclid(len as isize) as usize;
    state.select(Some(next));
}

fn toggle_done(state: &mut TableState, store: &mut Store) -> Result<()> {
    let Some(idx) = state.selected() else {
        return Ok(());
    };
    let id = match store.list(false, None).get(idx) {
        Some(task) => task.id,
        None => return Ok(()),
    };
    store.mark_done(id)?;
    Ok(())
}
