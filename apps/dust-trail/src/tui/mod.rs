//! TUI application: runs the game in the terminal with crossterm.
//! Turn-based: render → poll input → act → advance turn.

mod renderer;

use crate::domain::game::{Direction, Game, Interaction};
use crate::tui::renderer::render_frame;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use directories::ProjectDirs;
use std::io::{stdout, Stdout};
use std::path::{Path, PathBuf};
use std::time::Duration;

enum ControlFlow {
    Continue,
    Quit,
}

/// The TUI frontend.
pub struct Tui {
    stdout: Stdout,
    game: Game,
    save_dir: PathBuf,
}

impl Tui {
    pub fn new() -> std::io::Result<Self> {
        let save_dir = ProjectDirs::from("", "", "dust-trail")
            .map(|p| p.config_dir().to_path_buf())
            .unwrap_or_else(|| std::env::temp_dir().join("dust-trail"));
        std::fs::create_dir_all(&save_dir).ok();

        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        stdout.execute(Hide)?;

        Ok(Self {
            stdout,
            game: Game::new(),
            save_dir,
        })
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let result = self.loop_until_quit();
        // Always restore the terminal, even on error.
        let _ = self.stdout.execute(Show);
        let _ = self.stdout.execute(LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
        result
    }

    fn loop_until_quit(&mut self) -> std::io::Result<()> {
        loop {
            let frame = self.game.frame();
            render_frame(&mut self.stdout, &frame)?;

            // Poll briefly so the screen redraws (status/message updates) and
            // we stay responsive. 100 ms ≈ 10 fps; fine for a turn-based game.
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match self.handle_key(key)? {
                        ControlFlow::Quit => return Ok(()),
                        ControlFlow::Continue => {}
                    }
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> std::io::Result<ControlFlow> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Ok(ControlFlow::Quit);
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Ok(ControlFlow::Quit),
            KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                self.step(Direction::Up);
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                self.step(Direction::Down);
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                self.step(Direction::Left);
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                self.step(Direction::Right);
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('e') => {
                self.interact();
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('S') => {
                self.save_game();
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('l') => {
                self.load_game();
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('r') => {
                self.game = Game::new();
                Ok(ControlFlow::Continue)
            }
            _ => Ok(ControlFlow::Continue),
        }
    }

    fn step(&mut self, dir: Direction) {
        match self.game.move_player(dir) {
            Interaction::None => {}
            Interaction::Blocked { reason } => {
                self.game.message = format!("Blocked: {reason}");
            }
            Interaction::Collected { kind } => {
                self.game.message = format!("Picked up {:?}.", kind);
            }
            Interaction::Talked { npc, line } => {
                self.game.message = format!("{npc}: {line}");
            }
        }
    }

    fn interact(&mut self) {
        match self.game.interact() {
            Interaction::None => {}
            Interaction::Blocked { reason } => {
                self.game.message = format!("{reason}");
            }
            Interaction::Collected { kind } => {
                self.game.message = format!("Picked up {:?}.", kind);
            }
            Interaction::Talked { npc, line } => {
                self.game.message = format!("{npc}: {line}");
            }
        }
    }

    fn save_path(&self, name: &str) -> PathBuf {
        self.save_dir.join(format!("{name}.json"))
    }

    fn save_game(&mut self) {
        let path = self.save_path("savegame");
        if let Err(e) = self.game.save(&path) {
            self.game.message = format!("Save failed: {e}");
        }
    }

    fn load_game(&mut self) {
        let path = self.save_path("savegame");
        match Game::load(&path) {
            Ok(loaded) => self.game = loaded,
            Err(e) => self.game.message = format!("Load failed: {e}"),
        }
    }

    /// Prompt-less slot list: show available save files. Kept minimal for v0.1.0.
    pub fn save_dir(&self) -> &Path {
        &self.save_dir
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.stdout.execute(Show);
        let _ = self.stdout.execute(LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
