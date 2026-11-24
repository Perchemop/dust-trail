//! Save / load dialogs for the GUI frontend.
//!
//! Pure UI state and rendering helpers. All persistence goes through the
//! domain `SaveStore` / `SaveSlots` system; this module never touches the
//! file system directly.

use crate::domain::game::Game;
use crate::domain::slots::{SaveSlotMeta, SaveStore, SAVE_SLOTS_FILE_NAME};
use crate::theme::{danger_color32, Variant};
use eframe::egui;
use std::path::PathBuf;

/// Resolve the directory where saves live.
pub fn save_dir() -> PathBuf {
    crate::domain::slots::default_save_dir()
}

pub fn save_store() -> SaveStore {
    SaveStore::new(save_dir().join(SAVE_SLOTS_FILE_NAME))
}

/// State for the Save dialog.
#[derive(Default)]
pub struct SaveDialog {
    pub open: bool,
    pub slots: Vec<SaveSlotMeta>,
    pub selected: Option<String>,
    pub new_name: String,
    pub error: Option<String>,
    pub confirm_overwrite: bool,
    pub confirm_delete: Option<String>,
    pub pending_delete: Option<String>,
    pub loading: bool,
}

impl SaveDialog {
    pub fn open(&mut self) {
        self.open = true;
        self.error = None;
        self.confirm_overwrite = false;
        self.confirm_delete = None;
        self.pending_delete = None;
        self.refresh();
        if self.new_name.trim().is_empty() {
            self.new_name = "save_1".to_string();
        }
        self.selected = None;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.error = None;
        self.confirm_overwrite = false;
        self.confirm_delete = None;
        self.pending_delete = None;
    }

    pub fn refresh(&mut self) {
        match save_store().list_metadata() {
            Ok(meta) => self.slots = meta,
            Err(e) => {
                self.slots.clear();
                self.error = Some(format!("Error listing saves: {e}"));
            }
        }
    }

    /// Render the dialog. Returns `true` when a save was committed this frame.
    pub fn render(&mut self, ctx: &egui::Context, game: &mut Game) -> bool {
        if !self.open {
            return false;
        }
        // Keyboard navigation through the slot list.
        let slot_names: Vec<String> = self.slots.iter().map(|m| m.name.clone()).collect();
        if !slot_names.is_empty() {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                let idx = self
                    .selected
                    .as_ref()
                    .and_then(|s| slot_names.iter().position(|n| n == s))
                    .map(|i| (i + 1).min(slot_names.len() - 1))
                    .unwrap_or(0);
                self.selected = Some(slot_names[idx].clone());
                self.new_name = slot_names[idx].clone();
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                let idx = self
                    .selected
                    .as_ref()
                    .and_then(|s| slot_names.iter().position(|n| n == s))
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(slot_names.len() - 1);
                self.selected = Some(slot_names[idx].clone());
                self.new_name = slot_names[idx].clone();
            }
        }
        let mut committed = false;
        let mut is_open = self.open;
        egui::Window::new("Save Game")
            .open(&mut is_open)
            .default_width(360.0)
            .show(ctx, |ui| {
                ui.label("Slot name:");
                let _ = ui.add(egui::TextEdit::singleline(&mut self.new_name).hint_text("new slot name"));
                ui.add_space(4.0);

                // Existing slots list.
                ui.label("Existing slots:");
                egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .show(ui, |ui| {
                        if self.slots.is_empty() {
                            ui.weak("No saves yet.");
                        }
                        for meta in self.slots.clone() {
                            let selected = self.selected.as_deref() == Some(meta.name.as_str());
                            let pos_str = save_store()
                                .load_data(&meta.name)
                                .ok()
                                .map(|d| format!("{},{}", d.player.pos.x, d.player.pos.y))
                                .unwrap_or_else(|| "?".to_string());
                            let label = format!(
                                "{} — {} · turn {} · pos {} · score {}",
                                meta.name, meta.player_name, meta.turn, pos_str, meta.score
                            );
                            let resp = ui.selectable_label(selected, label);
                            if resp.clicked() {
                                self.selected = Some(meta.name.clone());
                                self.new_name = meta.name.clone();
                            }
                        }
                    });

                if let Some(err) = &self.error {
                    ui.colored_label(danger_color32(Variant::Dark), err);
                    if ui.small_button("Retry").clicked() {
                        self.error = None;
                        self.refresh();
                    }
                }
                if self.loading {
                    crate::theme::skeleton(ui, egui::vec2(320.0, 16.0));
                }

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let save_enabled = !self.new_name.trim().is_empty();
                    if ui
                        .add_enabled(save_enabled, egui::Button::new("Save"))
                        .clicked()
                    {
                        self.try_save(game, &mut committed);
                    }
                    if ui.button("Cancel").clicked() {
                        self.close();
                    }
                    // Delete selected slot.
                    if self.selected.is_some()
                        && ui.button("Delete").clicked()
                    {
                        self.pending_delete = self.selected.clone();
                        self.confirm_delete = self.pending_delete.clone();
                    }
                });

                // Overwrite confirmation.
                if self.confirm_overwrite {
                    ui.add_space(4.0);
                    ui.colored_label(
                        danger_color32(Variant::Dark),
                        format!(
                            "Slot \"{}\" already exists. Overwrite?",
                            self.new_name.trim()
                        ),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Overwrite").clicked() {
                            self.confirm_overwrite = false;
                            self.do_save(game, &mut committed);
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_overwrite = false;
                        }
                    });
                }

                // Delete confirmation.
                if let Some(name) = self.confirm_delete.clone() {
                    ui.add_space(4.0);
                    ui.colored_label(
                        danger_color32(Variant::Dark),
                        format!("Delete slot \"{}\"?", name),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Delete").clicked() {
                            match save_store().delete_save(&name) {
                                Ok(()) => {
                                    self.refresh();
                                    self.selected = None;
                                    self.new_name.clear();
                                    game.message = format!("Deleted slot \"{}\".", name);
                                }
                                Err(e) => self.error = Some(format!("Delete failed: {e}")),
                            }
                            self.confirm_delete = None;
                            self.pending_delete = None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.confirm_delete = None;
                            self.pending_delete = None;
                        }
                    });
                }
            });
        self.open = is_open;
        committed
    }

    fn try_save(&mut self, game: &mut Game, committed: &mut bool) {
        let slot = self.new_name.trim().to_string();
        if slot.is_empty() {
            self.error = Some("Slot name must not be empty.".to_string());
            return;
        }
        // If slot exists, ask for confirmation.
        if self.slots.iter().any(|m| m.name == slot) {
            self.confirm_overwrite = true;
            return;
        }
        self.do_save(game, committed);
    }

    fn do_save(&mut self, game: &mut Game, committed: &mut bool) {
        let slot = self.new_name.trim().to_string();
        match game.save_named_to(&slot, &save_dir().join(SAVE_SLOTS_FILE_NAME)) {
            Ok(()) => {
                let clone = slot.clone();
                self.refresh();
                self.open = false;
                self.new_name.clear();
                self.selected = None;
                game.message = format!("Game saved to slot \"{}\".", clone);
                *committed = true;
            }
            Err(e) => {
                self.error = Some(format!("Save failed: {e}"));
            }
        }
    }

    /// Handle Esc to close.
    pub fn handle_escape(&mut self) {
        if self.open {
            self.close();
        }
    }
}

/// State for the Load dialog.
#[derive(Default)]
pub struct LoadDialog {
    pub open: bool,
    pub slots: Vec<SaveSlotMeta>,
    pub selected: Option<String>,
    pub error: Option<String>,
    pub loading: bool,
}

impl LoadDialog {
    pub fn open(&mut self) {
        self.open = true;
        self.error = None;
        self.selected = None;
        self.refresh();
    }

    pub fn close(&mut self) {
        self.open = false;
        self.error = None;
    }

    pub fn refresh(&mut self) {
        match save_store().list_metadata() {
            Ok(meta) => self.slots = meta,
            Err(e) => {
                self.slots.clear();
                self.error = Some(format!("Error listing saves: {e}"));
            }
        }
    }

    /// Render the dialog. Returns the slot name loaded, if any.
    pub fn render(&mut self, ctx: &egui::Context, game: &mut Game) -> Option<String> {
        if !self.open {
            return None;
        }
        let slot_names: Vec<String> = self.slots.iter().map(|m| m.name.clone()).collect();
        if !slot_names.is_empty() {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                let idx = self
                    .selected
                    .as_ref()
                    .and_then(|s| slot_names.iter().position(|n| n == s))
                    .map(|i| (i + 1).min(slot_names.len() - 1))
                    .unwrap_or(0);
                self.selected = Some(slot_names[idx].clone());
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                let idx = self
                    .selected
                    .as_ref()
                    .and_then(|s| slot_names.iter().position(|n| n == s))
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(slot_names.len() - 1);
                self.selected = Some(slot_names[idx].clone());
            }
        }
        let mut loaded_slot: Option<String> = None;
        let mut is_open = self.open;
        egui::Window::new("Load Game")
            .open(&mut is_open)
            .default_width(360.0)
            .show(ctx, |ui| {
                ui.label("Select a save slot:");
                egui::ScrollArea::vertical()
                    .max_height(220.0)
                    .show(ui, |ui| {
                        if self.slots.is_empty() {
                            ui.weak("No saves yet.");
                        }
                        for meta in self.slots.clone() {
                            let selected = self.selected.as_deref() == Some(meta.name.as_str());
                            let pos_str = save_store()
                                .load_data(&meta.name)
                                .ok()
                                .map(|d| format!("{},{}", d.player.pos.x, d.player.pos.y))
                                .unwrap_or_else(|| "?".to_string());
                            let label = format!(
                                "{} — turn {} · pos {} · score {} · v{}",
                                meta.name, meta.turn, pos_str, meta.score, meta.version
                            );
                            let resp = ui.selectable_label(selected, label);
                            if resp.clicked() {
                                self.selected = Some(meta.name.clone());
                            }
                        }
                    });

                if let Some(err) = &self.error {
                    ui.colored_label(danger_color32(Variant::Dark), err);
                    if ui.small_button("Retry").clicked() {
                        self.error = None;
                        self.refresh();
                    }
                }
                if self.loading {
                    crate::theme::skeleton(ui, egui::vec2(320.0, 16.0));
                }

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let load_enabled = self.selected.is_some();
                    if ui
                        .add_enabled(load_enabled, egui::Button::new("Load"))
                        .clicked()
                    {
                        if let Some(slot) = self.selected.clone() {
                            match Game::load_named_from(&slot, &save_dir().join(SAVE_SLOTS_FILE_NAME))
                            {
                                Ok(loaded) => {
                                    *game = loaded;
                                    self.open = false;
                                    self.selected = None;
                                    game.message = "Game loaded.".to_string();
                                    loaded_slot = Some(slot);
                                }
                                Err(e) => {
                                    self.error = Some(format!("Load failed: {e}"));
                                }
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.close();
                    }
                });
            });
        self.open = is_open;
        loaded_slot
    }

    pub fn handle_escape(&mut self) {
        if self.open {
            self.close();
        }
    }
}
