//! TUI application: runs the game in the terminal with crossterm.
//! Turn-based: render → poll input → act → advance turn.

mod renderer;

use crate::domain::game::{Direction, Game, Interaction};
use crate::tui::renderer::render_frame;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use directories::ProjectDirs;
use std::io::{stdout, Stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

enum ControlFlow {
    Continue,
    Quit,
}

/// Pending action waiting for the user to select a save slot.
#[derive(Copy, Clone)]
enum PendingAction {
    SaveSlot,
    LoadSlot,
}

/// The TUI frontend.
pub struct Tui {
    stdout: Stdout,
    game: Game,
    save_dir: PathBuf,
    pending_action: Option<PendingAction>,
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
            pending_action: None,
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

            // Render pending action prompt (slot selection).
            if let Some(action) = self.pending_action {
                self.render_slot_prompt(&action)?;
                self.stdout.flush()?;
            }

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

    /// Render the slot selection prompt for a pending save/load action.
    fn render_slot_prompt(&mut self, action: &PendingAction) -> std::io::Result<()> {
        use crossterm::style::{Print, SetForegroundColor};
        use crossterm::{cursor::MoveTo, ExecutableCommand};

        let slots = Game::list_saves().unwrap_or_default();
        let action_word = match action {
            PendingAction::SaveSlot => "save",
            PendingAction::LoadSlot => "load",
        };

        self.stdout.execute(MoveTo(0, 0))?;
        self.stdout
            .execute(SetForegroundColor(crossterm::style::Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            }))?;
        self.stdout.execute(Print(format!(
            "Select slot to {} (1-9, Esc to cancel):\n",
            action_word
        )))?;
        for (i, slot) in slots.iter().enumerate().take(9) {
            self.stdout
                .execute(Print(format!("  {}. {}\n", i + 1, slot)))?;
        }
        if slots.is_empty() {
            self.stdout.execute(Print("  (no saves yet)\n"))?;
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> std::io::Result<ControlFlow> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Ok(ControlFlow::Quit);
        }

        // If waiting for slot selection, handle digit keys.
        if let Some(action) = self.pending_action.take() {
            if let KeyCode::Char(c) = key.code {
                if ('1'..='9').contains(&c) {
                    let slot_idx = c.to_digit(10).unwrap() as usize - 1;
                    let slots = Game::list_saves().unwrap_or_default();
                    if slot_idx < slots.len() {
                        let slot = &slots[slot_idx];
                        match action {
                            PendingAction::SaveSlot => {
                                let path = self.save_dir.join("saves.json");
                                match self.game.save_named_to(slot, &path) {
                                    Ok(()) => {
                                        self.game.message =
                                            format!("Game saved to slot \"{slot}\".");
                                    }
                                    Err(e) => {
                                        self.game.message = format!("Save failed: {e}");
                                    }
                                }
                            }
                            PendingAction::LoadSlot => {
                                let path = self.save_dir.join("saves.json");
                                match Game::load_named_from(slot, &path) {
                                    Ok(loaded) => self.game = loaded,
                                    Err(e) => {
                                        self.game.message = format!("Load failed: {e}");
                                    }
                                }
                            }
                        }
                        return Ok(ControlFlow::Continue);
                    }
                }
            }
            // Esc or any other key cancels.
            self.game.message = "Save/load cancelled.".to_string();
            return Ok(ControlFlow::Continue);
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
            KeyCode::Char('d') | KeyCode::Right => {
                self.step(Direction::Right);
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('e') => {
                self.interact();
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('S') => {
                let slots = Game::list_saves().unwrap_or_default();
                if slots.is_empty() {
                    // No saves yet, save to slot "1".
                    let path = self.save_dir.join("saves.json");
                    match self.game.save_named_to("1", &path) {
                        Ok(()) => {
                            self.game.message = "Game saved to slot \"1\".".to_string();
                        }
                        Err(e) => {
                            self.game.message = format!("Save failed: {e}");
                        }
                    }
                } else {
                    self.pending_action = Some(PendingAction::SaveSlot);
                }
                Ok(ControlFlow::Continue)
            }
            KeyCode::Char('l') => {
                let slots = Game::list_saves().unwrap_or_default();
                if slots.is_empty() {
                    self.game.message = "No saves to load.".to_string();
                } else {
                    self.pending_action = Some(PendingAction::LoadSlot);
                }
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
                self.game.message = reason.to_string();
            }
            Interaction::Collected { kind } => {
                self.game.message = format!("Picked up {:?}.", kind);
            }
            Interaction::Talked { npc, line } => {
                self.game.message = format!("{npc}: {line}");
            }
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
