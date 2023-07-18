
use std::collections::{VecDeque, BTreeMap};

use bevy::{prelude::*};
use bevy_egui::egui::{Context, Ui, RichText, Color32};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::mrr::{Assembly};
use crate::log::{LogMessages, LogMessageType};

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

#[derive(Default)]
struct FilePanel;

impl FilePanel {
    fn ui(&mut self, ui: &mut Ui, mut assembly: ResMut<Assembly>, log: &mut LogMessages) {
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

            assembly.file_path = path;

            assembly.load_meshes();
            println!("Import Robot");
        }

        let response = ui.button("Delete Robot");
        if response.clicked() {
            println!("{}", assembly.get_name());
            log.error("Robot Deleted", "Robot was unexpectedly deleted.", true);
        }
        
        if ui.button("Warn").clicked() {
            log.warn("Some Warning", "This is a warning.");
        }

        if ui.button("Info").clicked() {
            log.info("Info Message", "This is an info message.");
        }
    }

    fn windows(&mut self, ui: &mut Ui) {

    }
}

struct HelpPanel {
    about_mechsim_open: bool,
    mechsim_logo_texture: Option<egui_extras::RetainedImage>,
}

impl Default for HelpPanel {
    fn default() -> Self {
        Self {
            about_mechsim_open: false,
            mechsim_logo_texture: egui_extras::RetainedImage::from_svg_bytes_with_size(
                "mechsim-logo.svg",
                include_bytes!("mechsim-logo.svg"),
                egui_extras::image::FitTo::Zoom(0.15)
            ).ok()
        }
    }
}

impl HelpPanel {
    fn ui(&mut self, ui: &mut Ui, mut log: &LogMessages) {
        if ui.button("About MechSim").clicked() {
            self.about_mechsim_open = !self.about_mechsim_open;
        }
    }

    fn windows(&mut self, ui: &mut Ui) {
        self.about_window(ui);
    }

    fn about_window(&mut self, ui: &mut Ui) {
        use egui::special_emojis::GITHUB;

        egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .open(&mut self.about_mechsim_open)
        .show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.);

                if let Some(logo) = &self.mechsim_logo_texture {
                    logo.show(ui);
                }

                ui.add_space(12.);
                ui.heading(RichText::new("MechSim").size(28.).strong());
                ui.add_space(12.);

                ui.label(RichText::new("A 3D robot visualizer, physics simulator, and robot code verification tool.").size(14.));
                ui.add_space(10.);
                ui.hyperlink_to(
                    format!("{} mechsim on GitHub", GITHUB),
                        "https://github.com/mechsimulator/mechsim"
                    );
            });
        });
    }
}

#[derive(Resource, Default)]
pub struct MenuBar {
    file_panel: FilePanel,
    help_panel: HelpPanel,
}

impl MenuBar {
    fn windows(&mut self, ui: &mut Ui) {
        self.file_panel.windows(ui);
        self.help_panel.windows(ui);
    }
}

fn menu_bar_system(
    mut contexts: EguiContexts,
    mut menu_bar: ResMut<MenuBar>,
    assembly: ResMut<Assembly>,
    mut log: ResMut<LogMessages>
) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                menu_bar.file_panel.ui(ui, assembly, log.as_mut());
            });

            ui.menu_button("Help", |ui| {
                menu_bar.help_panel.ui(ui, log.as_mut());
            });
            menu_bar.windows(ui); 
        });
    });
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
        .init_resource::<BottomPanel>()
        .init_resource::<MenuBar>()
        .add_system(bottom_panel_system)
        .add_system(menu_bar_system);
    }
}
