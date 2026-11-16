//! GUI frontend: renders the domain `Frame` as monospace ASCII art inside
//! egui/eframe 0.37. Feature-gated behind the `gui` Cargo feature.
//!
//! Clean-room: original layout and ASCII style.

use crate::domain::cell::Cell;
use crate::domain::game::{Direction, Game, Interaction};
use crate::theme::{danger_color32, rgb_to_color32, Variant};
use eframe::egui;

#[cfg(feature = "gui")]
use accesskit::NodeBuilder;

/// The GUI frontend.
pub struct Gui {
    game: Game,
    save_load: SaveLoadState,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Enable AccessKit for screen reader support (guide §II.21)
        #[cfg(feature = "gui")]
        {
            cc.egui_ctx.set_accesskit_adapter(Some(Box::new(
                accesskit_egui::AccessKitAdapter::new()
            )));
        }

        Self {
            game: Game::new(),
            save_load: SaveLoadState::default(),
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Dust Trail");
            ui.separator();
            if ui.add(egui::Button::new("New Game")
                .accessibility_label("Start a new game")).clicked() {
                self.game = Game::new();
            }
            if ui.add(egui::Button::new("Save")
                .accessibility_label("Save current game")).clicked() {
                self.save_load.open_save();
            }
            if ui.add(egui::Button::new("Load")
                .accessibility_label("Load a saved game")).clicked() {
                self.save_load.open_load();
            }
            if ui.add(egui::Button::new("Quit")
                .accessibility_label("Quit the game")).clicked() {
                ctx_close(ui.ctx());
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
    }

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
                    egui::Key::Q | egui::Key::Escape => {
                        ctx_close(ctx);
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

    fn save_game(&mut self) {
        let path = save_dir().join("saves.json");
        let slot = self.save_load.save_name_input.trim().to_string();
        if slot.is_empty() {
            self.game.message = "Save slot name cannot be empty.".to_string();
            return;
        }
        if let Err(e) = self.game.save_named_to(&slot, &path) {
            self.game.message = format!("Save failed: {e}");
        } else {
            let slot_clone = slot.clone();
            self.save_load.last_saved_slot = Some(slot);
            self.save_load.is_save_dialog_open = false;
            self.save_load.save_name_input.clear();
            self.game.message = format!("Game saved to slot \"{slot_clone}\".");
        }
    }

    fn load_game(&mut self) {
        let path = save_dir().join("saves.json");
        if let Some(slot) = &self.save_load.selected_slot {
            match Game::load_named_from(slot, &path) {
                Ok(loaded) => {
                    self.game = loaded;
                    self.save_load.last_saved_slot = Some(slot.clone());
                    self.save_load.is_load_dialog_open = false;
                    self.save_load.selected_slot = None;
                    self.game.message = "Game loaded.".to_string();
                }
                Err(e) => {
                    self.save_load.error = Some(format!("Load failed: {e}"));
                }
            }
        }
    }

    fn render_save_dialog(&mut self, ui: &mut egui::Ui) {
        if !self.save_load.is_save_dialog_open {
            return;
        }

        let mut is_open = self.save_load.is_save_dialog_open;
        egui::Window::new("Save Game")
            .open(&mut is_open)
            .default_width(320.0)
            .show(ui.ctx(), |ui| {
                ui.label("Save slot name:");
                ui.add(egui::TextEdit::singleline(&mut self.save_load.save_name_input)
                    .accessibility_label("Enter a name for the save slot"));
                ui.add_space(8.0);
                ui.label("Existing slots:");
                egui::ScrollArea::vertical()
                    .max_height(160.0)
                    .show(ui, |ui| {
                        match Game::list_saves() {
                            Ok(slots) => {
                                if slots.is_empty() {
                                    ui.weak("No saves yet.");
                                }
                                for slot in &slots {
                                    let selected = self.save_load.selected_slot.as_deref()
                                        == Some(slot.as_str());
                                    if ui.add(egui::SelectableLabel::new(selected, slot)
                                            .accessibility_label(format!("Save slot: {}", slot)))
                                        .clicked()
                                    {
                                        self.save_load.selected_slot = Some(slot.clone());
                                        self.save_load.save_name_input = slot.clone();
                                    }
                                }
                            }
                            Err(e) => {
                                ui.colored_label(
                                    danger_color32(Variant::Dark),
                                    format!("Error listing saves: {e}"),
                                );
                            }
                        }

                        if let Some(err) = &self.save_load.error {
                            ui.colored_label(danger_color32(Variant::Dark), err);
                        }

                        ui.add_space(8.0);
                        ui.columns(2, |uis| self.render_save_dialog_buttons(uis));
                    });
            });
        self.save_load.is_save_dialog_open = is_open;
    }

    fn render_save_dialog_buttons(&mut self, uis: &mut [egui::Ui]) {
        uis[0].vertical(|ui| {
            let save_enabled = !self.save_load.save_name_input.trim().is_empty();
            if ui
                .add_enabled(save_enabled, egui::Button::new("Save")
                    .accessibility_label("Save the game to the specified slot"))
                .clicked()
            {
                let slot = self.save_load.save_name_input.trim().to_string();
                match self
                    .game
                    .save_named_to(&slot, &save_dir().join("saves.json"))
                {
                    Ok(()) => {
                        let slot_clone = slot.clone();
                        self.save_load.last_saved_slot = Some(slot);
                        self.save_load.is_save_dialog_open = false;
                        self.save_load.save_name_input.clear();
                        self.game.message = format!("Game saved to slot \"{}\".", slot_clone);
                    }
                    Err(e) => {
                        self.save_load.error = Some(format!("Save failed: {e}"));
                    }
                }
            }
        });
        uis[1].vertical(|ui| {
            if ui.add(egui::Button::new("Cancel")
                    .accessibility_label("Cancel saving and close dialog"))
                .clicked()
            {
                self.save_load.close_all();
            }
        });
    }

    fn render_load_dialog_buttons(&mut self, uis: &mut [egui::Ui]) {
        uis[0].vertical(|ui| {
            let load_enabled = self.save_load.selected_slot.is_some();
            if ui
                .add_enabled(load_enabled, egui::Button::new("Load")
                    .accessibility_label("Load the selected saved game"))
                .clicked()
            {
                if let Some(slot) = &self.save_load.selected_slot {
                    match Game::load_named_from(slot, &save_dir().join("saves.json")) {
                        Ok(loaded) => {
                            self.game = loaded;
                            self.save_load.last_saved_slot = Some(slot.clone());
                            self.save_load.is_load_dialog_open = false;
                            self.save_load.selected_slot = None;
                            self.game.message = "Game loaded.".to_string();
                        }
                        Err(e) => {
                            self.save_load.error = Some(format!("Load failed: {e}"));
                        }
                    }
                }
            }
        });
        uis[1].vertical(|ui| {
            if ui.add(egui::Button::new("Cancel")
                    .accessibility_label("Cancel loading and close dialog"))
                .clicked()
            {
                self.save_load.close_all();
            }
        });
    }

    fn render_load_dialog(&mut self, ui: &mut egui::Ui) {
        if !self.save_load.is_load_dialog_open {
            return;
        }

        let mut is_open = self.save_load.is_load_dialog_open;
        egui::Window::new("Load Game")
            .open(&mut is_open)
            .default_width(320.0)
            .show(ui.ctx(), |ui| {
                ui.label("Select a save slot:");
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        match Game::list_saves() {
                            Ok(slots) => {
                                if slots.is_empty() {
                                    ui.weak("No saves yet.");
                                }
                                for slot in &slots {
                                    let selected = self.save_load.selected_slot.as_deref()
                                        == Some(slot.as_str());
                                    if ui.add(egui::SelectableLabel::new(selected, slot)
                                            .accessibility_label(format!("Save slot: {}", slot)))
                                        .clicked()
                                    {
                                        self.save_load.selected_slot = Some(slot.clone());
                                    }
                                }
                            }
                            Err(e) => {
                                ui.colored_label(
                                    danger_color32(Variant::Dark),
                                    format!("Error loading saves: {e}"),
                                );
                            }
                        }

                        if let Some(err) = &self.save_load.error {
                            ui.colored_label(danger_color32(Variant::Dark), err);
                        }

                        ui.add_space(8.0);
                        ui.columns(2, |uis| self.render_load_dialog_buttons(uis));
                    });
            });
        self.save_load.is_load_dialog_open = is_open;
    }
}

/// State for save/load dialogs.
#[derive(Default)]
pub struct SaveLoadState {
    is_save_dialog_open: bool,
    is_load_dialog_open: bool,
    save_name_input: String,
    selected_slot: Option<String>,
    error: Option<String>,
    last_saved_slot: Option<String>,
}

impl SaveLoadState {
    pub fn open_save(&mut self) {
        self.is_save_dialog_open = true;
        self.is_load_dialog_open = false;
        self.error = None;
        if self.save_name_input.trim().is_empty() {
            self.save_name_input = "save_1".to_string();
        }
        self.selected_slot = None;
    }

    pub fn open_load(&mut self) {
        self.is_load_dialog_open = true;
        self.is_save_dialog_open = false;
        self.error = None;
        self.selected_slot = None;
    }

    pub fn close_all(&mut self) {
        self.is_save_dialog_open = false;
        self.is_load_dialog_open = false;
        self.error = None;
    }
}

/// Resolve the save directory (config dir for dust-trail).
fn save_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "dust-trail")
        .map(|p| p.config_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("dust-trail"))
}

fn ctx_close(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        self.handle_keyboard(ctx);
        self.toolbar(ui);
        // Render the game grid directly in the central panel area
        let frame = self.game.frame();
        for (y, row) in frame.rows() {
            self.render_row(ui, row);
        }
        self.render_save_dialog(ui);
        self.render_load_dialog(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_color32_roundtrip() {
        let _ = crate::theme::rgb_to_color32(crate::domain::color::Rgb::BLACK);
    }
}
