
use std::collections::{VecDeque, BTreeMap};

use bevy::{prelude::*};
use bevy_egui::egui::{Context, Ui, RichText, Color32};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::mrr::{Assembly, AssemblyMeshes, AssemblyMetadata};
use crate::log::{LogMessages, LogMessageType};
pub struct UIPlugin;

#[derive(Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Log,
    Model,
    Physics,
}

#[derive(Resource, Default)]
pub struct BottomPanel {
    log_tab: LogTab,
    open: Tab,
}

impl BottomPanel {
    fn ui(&mut self, ui: &mut Ui, log: ResMut<LogMessages>) { 
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.open, Tab::Log, {
                if log.msgs.is_empty() {
                    "Log".to_owned()
                } else {
                    format!("Log ({})", log.msgs.len())
                }
            });
            ui.selectable_value(&mut self.open, Tab::Model, "Model");
            ui.selectable_value(&mut self.open, Tab::Physics, "Physics");
        });
        ui.separator();

        match self.open {
            Tab::Log => self.log_tab.ui(ui, log),
            Tab::Model => (),
            Tab::Physics => (),
        };
    }
}

#[derive(Default)]
struct LogTab;

impl LogTab {
    fn ui(&mut self, ui: &mut Ui, mut log: ResMut<LogMessages>) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut delete: (usize, bool) = Default::default();
            for (i, msg) in log.msgs.iter().rev().enumerate() {
                let title_text = RichText::new(&msg.title).color(msg.msg_type.color());
                
                let delete_idx = log.msgs.len() - i - 1;
                if let LogMessageType::Error { popup } = msg.msg_type {
                    if popup {
                        egui::Window::new(title_text.clone())
                        .resizable(false)
                        .collapsible(false)
                        .default_pos(ui.ctx().screen_rect().center())
                        .show(ui.ctx(), |ui| {
                            ui.label(&msg.msg);
                            if ui.button("Close").clicked() {
                                delete.1 = true;
                                delete.0 = delete_idx      
                            }
                        });
                    }
                }
                
                if ui.button(title_text.clone()).clicked() {
                    delete.1 = true;
                    delete.0 = delete_idx;
                }
                
                ui.label(&msg.msg);
                if i != log.msgs.len() - 1 {
                    ui.separator();
                }
            }

            if delete.1 {
                log.msgs.remove(delete.0);
            }
        });
    }
}

fn bottom_panel_system(
    mut contexts: EguiContexts,
    mut bottom_panel: ResMut<BottomPanel>,
    log: ResMut<LogMessages>
) {
    egui::TopBottomPanel::bottom("bottom_panel")
    .resizable(true)
    .show(contexts.ctx_mut(), |ui| {
        bottom_panel.ui(ui, log);
    });
}

fn menu_bar_system(
    mut contexts: EguiContexts,
    assembly: Res<Assembly>,
    mut assembly_meshes: ResMut<AssemblyMeshes>,
    mut assembly_metadata: ResMut<AssemblyMetadata>,
    mut log: ResMut<LogMessages>
) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Import Robot").clicked() {
                    let path = FileDialog::new()
                        .set_location("C:\\Users\\Public\\MechSim\\assemblies")
                        .add_filter("MRR Robot Description", &["mrr"])
                        .show_open_single_file()
                        .unwrap();

                    let path = match path {
                        Some(path) => path,
                        None => return,
                    };

                    assembly_metadata.file_path = path;

                    assembly_meshes.load_meshes(&assembly);
                    println!("Import Robot");
                }

                let response = ui.button("Delete Robot");
                if response.clicked() {
                    println!("{}", assembly_metadata.get_name());
                    log.error("Robot Deleted", "Robot was unexpectedly deleted.", true);
                }
                
                if ui.button("Warn").clicked() {
                    log.warn("Some Warning", "This is a warning.");
                }

                if ui.button("Info").clicked() {
                    log.info("Info Message", "This is an info message.");
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.add(egui::Button::new("ℹ Errors")
                .fill(egui::Color32::from_rgb(150, 100, 100).gamma_multiply(0.5)));
            });
        });
    });
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
        .init_resource::<BottomPanel>()
        .add_system(bottom_panel_system)
        .add_system(menu_bar_system);
    }
}
