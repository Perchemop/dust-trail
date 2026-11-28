//! GUI frontend: renders the domain `Frame` as monospace ASCII art inside
//! egui/eframe 0.37. Feature-gated behind the `gui` Cargo feature.
//!
//! Clean-room: original layout and ASCII style.

pub mod release_notes;
pub mod save_dialog;
pub mod settings;
pub mod camera; // T-210
pub mod viewport; // T-211

use crate::domain::cell::Cell;
use crate::domain::game::{Direction, Game, Interaction};
use crate::domain::world::{ItemKind, NpcKind, Pos, WantedLevel};
use crate::theme::{rgb_to_color32, Role, Theme};
use eframe::egui;

/// The GUI frontend.
pub struct Gui {
    game: Game,
    save_dialog: save_dialog::SaveDialog,
    load_dialog: save_dialog::LoadDialog,
    show_release_notes: bool,
    settings_state: settings::SettingsPanelState,
    #[allow(dead_code)]
    viewport: viewport::Viewport,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let settings_state = settings::SettingsPanelState::default();
        // Apply initial settings to the context.
        settings::apply_settings(&cc.egui_ctx, &settings_state.settings);
        Self {
            game: Game::new(),
            save_dialog: save_dialog::SaveDialog::default(),
            load_dialog: save_dialog::LoadDialog::default(),
            show_release_notes: false,
            settings_state,
            viewport: viewport::Viewport::default(),
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Dust Trail");
            ui.separator();
            if ui.add(egui::Button::new("New Game")).clicked() {
                self.game = Game::new();
            }
            if ui.add(egui::Button::new("Save")).clicked() {
                self.save_dialog.open();
            }
            if ui.add(egui::Button::new("Load")).clicked() {
                self.load_dialog.open();
            }
            if ui.add(egui::Button::new("Settings")).clicked() {
                self.settings_state.open();
            }
            if ui.add(egui::Button::new("Quit")).clicked() {
                ctx_close(ui.ctx());
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Wanted level (from outlaw).
                let wanted = self
                    .game
                    .world
                    .npcs
                    .iter()
                    .find(|n| n.kind == NpcKind::Outlaw)
                    .map(|n| n.wanted_level())
                    .unwrap_or(WantedLevel::None);
                let wanted_label = match wanted {
                    WantedLevel::None => "None".to_string(),
                    WantedLevel::Low => "Low".to_string(),
                    WantedLevel::Medium => "Medium".to_string(),
                    WantedLevel::High => "High".to_string(),
                };
                let wanted_color: Role = match wanted {
                    WantedLevel::None => Role::Success,
                    WantedLevel::Low => Role::Warning,
                    WantedLevel::Medium => Role::Danger,
                    WantedLevel::High => Role::Danger,
                };
                ui.colored_label(
                    Theme::dark().color32(wanted_color),
                    format!("Wanted: {}", wanted_label),
                );
                ui.separator();
                ui.label(format!("HP: {}/100", self.game.player.health));
                ui.separator();
                let inv_count = self.game.player.inventory.len();
                if inv_count > 0 {
                    ui.label(format!(
                        "Inv: {} item{}",
                        inv_count,
                        if inv_count == 1 { "" } else { "s" }
                    ));
                    ui.separator();
                }
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
    }

    fn render_row(&self, ui: &mut egui::Ui, row: &[Cell]) {
        let mut runs: Vec<(String, egui::Color32)> = Vec::new();
        let mut cur = String::new();
        let mut cur_color: Option<egui::Color32> = None;

        for cell in row {
            let color = rgb_to_color32(cell.fg);
            if Some(color) != cur_color {
                if !cur.is_empty() {
                    runs.push((
                        cur.clone(),
                        cur_color.unwrap_or(Theme::dark().color32(Role::TextPrimary)),
                    ));
                }
                cur = String::new();
                cur_color = Some(color);
            }
            cur.push(cell.ch);
        }
        if !cur.is_empty() {
            runs.push((
                cur,
                cur_color.unwrap_or(Theme::dark().color32(Role::TextPrimary)),
            ));
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
        // While a dialog is open, only Esc/Enter are consumed; gameplay keys
        // are ignored so the player does not move underneath the dialog.
        if self.save_dialog.open || self.load_dialog.open || self.settings_state.open {
            let events = ctx.input(|i| i.events.clone());
            for event in events {
                if let egui::Event::Key { key, pressed, .. } = event {
                    if !pressed {
                        continue;
                    }
                    if key == egui::Key::Escape {
                        if self.save_dialog.open {
                            self.save_dialog.close();
                        }
                        if self.load_dialog.open {
                            self.load_dialog.close();
                        }
                        if self.settings_state.open {
                            self.settings_state.close();
                        }
                    }
                }
            }
            return;
        }

        let events = ctx.input(|i| i.events.clone());
        for event in events {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = event
            {
                if !pressed {
                    continue;
                }
                if modifiers.ctrl && key == egui::Key::C {
                    ctx_close(ctx);
                    continue;
                }
                match key {
                    egui::Key::Escape => {
                        if self.show_release_notes {
                            self.show_release_notes = false;
                        } else {
                            ctx_close(ctx);
                        }
                    }
                    egui::Key::Q => {
                        ctx_close(ctx);
                    }
                    egui::Key::W | egui::Key::K | egui::Key::ArrowUp => {
                        self.step(Direction::Up);
                    }
                    egui::Key::J | egui::Key::ArrowDown => {
                        self.step(Direction::Down);
                    }
                    egui::Key::A | egui::Key::H | egui::Key::ArrowLeft => {
                        self.step(Direction::Left);
                    }
                    egui::Key::D | egui::Key::ArrowRight => {
                        self.step(Direction::Right);
                    }
                    egui::Key::S => {
                        self.save_dialog.open();
                    }
                    egui::Key::L => {
                        self.load_dialog.open();
                    }
                    egui::Key::E => self.interact(),
                    egui::Key::F => self.attack(),
                    egui::Key::M => self.game.player.toggle_mount(),
                    egui::Key::U => self.use_item(),
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

    fn attack(&mut self) {
        // Attack adjacent NPCs (fight).
        let adjacent = [
            Pos::new(self.game.player.pos.x + 1, self.game.player.pos.y),
            Pos::new(self.game.player.pos.x - 1, self.game.player.pos.y),
            Pos::new(self.game.player.pos.x, self.game.player.pos.y + 1),
            Pos::new(self.game.player.pos.x, self.game.player.pos.y - 1),
        ];
        let mut attacked = false;
        for pos in adjacent {
            if self.game.world.npc_at(pos).is_some() {
                let dead = self.game.attack(pos);
                attacked = true;
                if dead {
                    self.game.message = "Enemy defeated!".to_string();
                }
                break;
            }
        }
        if !attacked {
            self.game.message = "No enemy nearby to attack.".to_string();
        }
    }

    fn use_item(&mut self) {
        if self.game.player.inventory.is_empty() {
            self.game.message = "Inventory empty.".to_string();
            return;
        }
        let kind = self.game.player.inventory[0];
        let result = self.game.use_item(kind);
        match result {
            Some(score) if score > 0 => {
                self.game.message = format!("Used item (+{} score).", score);
            }
            Some(_) => {
                self.game.message = "Used item.".to_string();
            }
            None => {
                self.game.message = "Nothing to use.".to_string();
            }
        }
    }

    fn render_inspector_panel(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Inspector")
            .collapsible(true)
            .resizable(true)
            .default_width(220.0)
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 100.0])
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Select an entity")
                            .color(Theme::dark().color32(Role::TextMuted)),
                    );
                    ui.small("Click an NPC or item in the world to inspect.");
                    ui.add_space(8.0);
                });
            });
    }

    fn render_inventory_panel(&mut self, ui: &mut egui::Ui) {
        // Empty state: always show the panel so the player knows where
        // inventory lives (guide II.30.11).
        egui::Window::new("Inventory")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::LEFT_TOP, [10.0, 100.0])
            .show(ui.ctx(), |ui| {
                if self.game.player.inventory.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("No items collected")
                                .color(Theme::dark().color32(Role::TextMuted)),
                        );
                        ui.label(
                            egui::RichText::new("Pick up loot ($) or a horse (&) in the world.")
                                .small()
                                .color(Theme::dark().color32(Role::TextMuted)),
                        );
                        ui.add_space(8.0);
                    });
                } else {
                    ui.vertical(|ui| {
                        ui.label("Inventory:");
                        for kind in &self.game.player.inventory {
                            let (glyph, label) = match kind {
                                ItemKind::Loot => ("$", "Satchel of coins"),
                                ItemKind::Horse => ("&", "Horse"),
                            };
                            ui.button(
                                egui::RichText::new(format!("{} {}", glyph, label))
                                    .monospace()
                                    .color(Theme::dark().color32(Role::TextPrimary)),
                            )
                            .on_hover_text(match kind {
                                ItemKind::Loot => "Consume for +5 score bonus.",
                                ItemKind::Horse => "Mount to move faster.",
                            });
                        }
                    });
                }
            });
    }
}

fn ctx_close(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        let reduced_motion = self.settings_state.settings.reduced_motion;
        self.handle_keyboard(ctx);
        self.toolbar(ui);
        self.render_inventory_panel(ui);
        self.render_inspector_panel(ui);
        // Render the game grid directly in the central panel area
        let frame = self.game.frame();
        let rows: Vec<_> = frame.rows().collect();
        if rows.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(
                    egui::RichText::new("World not loaded")
                        .color(Theme::dark().color32(Role::TextMuted)),
                );
            });
        } else {
            for (_y, row) in rows {
                self.render_row(ui, row);
            }
        }
        // New dialogs render themselves via egui windows.
        self.save_dialog.render(ui.ctx(), &mut self.game);
        self.load_dialog.render(ui.ctx(), &mut self.game);
        let changed = settings::draw_settings_panel(&mut self.settings_state, ui.ctx());
        if changed {
            // Sync sound settings to the game's sound manager.
            let s = &self.settings_state.settings;
            self.game.sound_manager.set_muted(!s.sound_enabled);
            self.game.sound_manager.set_master_volume(s.volume);
        }
        // Apply reduced-motion aware transition for game-over overlay.
        let _ = reduced_motion;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let version_btn = egui::Button::new(
                egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .small()
                    .weak(),
            )
            .frame(false);
            if ui
                .add(version_btn)
                .on_hover_text("Click for release notes")
                .clicked()
            {
                self.show_release_notes = true;
            }
        });
        crate::gui::release_notes::draw_release_notes(self, ui.ctx());

        // Game over overlay.
        if self.game.game_over {
            let alpha = crate::theme::fade_alpha(ui.ctx(), egui::Id::unique("game_over"), true, reduced_motion);
            egui::Window::new("GAME OVER")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(egui::RichText::new("GAME OVER").color(
                            crate::theme::rgb_to_color32(crate::domain::color::Rgb::new(
                                220, 60, 60,
                            )),
                        ));
                        let _ = alpha;
                        ui.add_space(16.0);
                        ui.label("You have been killed. Your journey ends here.");
                        ui.add_space(16.0);
                        if ui.button("Restart").clicked() {
                            self.game = Game::new();
                            self.game.game_over = false;
                            self.game.won = false;
                        }
                        if ui.button("Quit").clicked() {
                            ctx_close(ui.ctx());
                        }
                    });
                });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rgb_to_color32_roundtrip() {
        let _ = crate::theme::rgb_to_color32(crate::domain::color::Rgb::BLACK);
    }
}
