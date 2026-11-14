//! GUI frontend: renders the domain `Frame` as monospace ASCII art inside
//! egui/eframe. Feature-gated behind the `gui` Cargo feature.
//!
//! Clean-room: original layout and ASCII style.

use crate::domain::game::{Direction, Game, Interaction};
use crate::domain::cell::Cell;
use crate::theme::{rgb_to_color32, Role, Theme};
use eframe::egui;

/// The GUI frontend.
pub struct Gui {
    game: Game,
    theme: Theme,
}

impl Gui {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            game: Game::new(),
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.handle_keyboard(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel(ui);
        });
        // Redraw periodically so status/message updates are visible.
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

impl Gui {
    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Dust Trail");
                ui.separator();
                if ui.button("New Game").clicked() {
                    self.game = Game::new();
                }
                if ui.button("Save").clicked() {
                    self.save_game();
                }
                if ui.button("Load").clicked() {
                    self.load_game();
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "Turn: {}  Score: {}  {}",
                        self.game.turn,
                        self.game.player.score,
                        if self.game.player.mounted {
                            "[mounted]"
                        } else {
                            "[on foot]"
                        }
                    ));
                });
            });
        });
    }

    fn central_panel(&mut self, ui: &mut egui::Ui) {
        let frame = self.game.frame();

        egui::ScrollArea::both().show(ui, |ui| {
            for (_y, row) in frame.rows() {
                ui.horizontal(|ui| {
                    self.render_row(ui, row);
                });
                // A zero-height label forces a line break between rows.
                ui.label("");
            }
        });

        // Hint line below the grid.
        ui.separator();
        ui.label(
            "WASD / arrow keys to move  •  E to interact  •  Save & Load via toolbar  •  Quit to exit",
        );
        if self.game.game_over {
            ui.colored_label(
                self.theme.color32(Role::Danger),
                if self.game.won {
                    "Outlaw captured — town is safe! Press New Game to play again."
                } else {
                    "The outlaw escaped. Press New Game to play again."
                },
            );
        }
    }

    /// Render one row of cells, grouping consecutive same-colour cells into a
    /// single monospace label so the grid stays perfectly aligned.
    fn render_row(&self, ui: &mut egui::Ui, row: &[Cell]) {
        let mut runs: Vec<(String, egui::Color32)> = Vec::new();
        let mut cur = String::new();
        let mut cur_color: Option<egui::Color32> = None;

        for cell in row {
            let color = rgb_to_color32(cell.fg);
            if Some(color) != cur_color {
                if !cur.is_empty() {
                    runs.push((cur.clone(), cur_color.unwrap_or(egui::Color32::WHITE)));
                }
                cur = String::new();
                cur_color = Some(color);
            }
            cur.push(cell.ch);
        }
        if !cur.is_empty() {
            runs.push((cur, cur_color.unwrap_or(egui::Color32::WHITE)));
        }

        for (text, color) in runs {
            ui.label(
                egui::RichText::new(text)
                    .monospace()
                    .color(color)
                    .size(13.0),
            );
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        let events = ctx.input(|i| i.events.clone());
        for event in events {
            if let egui::Event::Key {
                key, pressed, modifiers, ..
            } = event
            {
                if !*pressed {
                    continue;
                }
                if modifiers.ctrl && key == egui::Key::C {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    continue;
                }
                match key {
                    egui::Key::Q | egui::Key::Escape => {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    egui::Key::W | egui::Key::K | egui::Key::ArrowUp => {
                        self.step(Direction::Up);
                    }
                    egui::Key::S | egui::Key::J | egui::Key::ArrowDown => {
                        self.step(Direction::Down);
                    }
                    egui::Key::A | egui::Key::H | egui::Key::ArrowLeft => {
                        self.step(Direction::Left);
                    }
                    egui::Key::D | egui::Key::L | egui::Key::ArrowRight => {
                        self.step(Direction::Right);
                    }
                    egui::Key::E => self.interact(),
                    egui::Key::R => self.game = Game::new(),
                    _ => {}
                }
            }
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

    fn save_game(&mut self) {
        let path = save_dir().join("savegame.json");
        if let Err(e) = self.game.save(&path) {
            self.game.message = format!("Save failed: {e}");
        } else {
            self.game.message = "Game saved.".to_string();
        }
    }

    fn load_game(&mut self) {
        let path = save_dir().join("savegame.json");
        match Game::load(&path) {
            Ok(loaded) => self.game = loaded,
            Err(e) => self.game.message = format!("Load failed: {e}"),
        }
    }
}

/// Resolve the save directory (config dir for dust-trail).
fn save_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "dust-trail")
        .map(|p| p.config_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("dust-trail"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_color32_roundtrip() {
        let col = Rgb::new(10, 20, 30);
        let c = rgb_to_color32(col);
        assert_eq!((c.r(), c.g(), c.b()), (10, 20, 30));
    }
}
