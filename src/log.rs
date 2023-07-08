use std::{fs::File, io::Write, fmt};

use bevy::prelude::*;
use bevy_egui::egui::Color32;
use chrono::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub enum LogMessageType {
    #[default]
    Info,
    Warning,
    Error {
        popup: bool
    },
}

impl fmt::Display for LogMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            LogMessageType::Info => "INFO",
            LogMessageType::Warning => "WARNING",
            LogMessageType::Error { popup } => if popup { "ERROR(popup)" } else { "ERROR" },
        })
    }
}

impl LogMessageType {
    pub fn color(&self) -> Color32 {
        match self {
            LogMessageType::Info => Color32::GRAY,
            LogMessageType::Warning => Color32::YELLOW,
            LogMessageType::Error { popup: _ } => Color32::RED,
        }
    }
}

#[derive(Resource, Debug)]
pub struct LogMessages {
    pub msgs: Vec<LogMessage>,
    log_file: Option<File>,
}

#[derive(Default, Debug)]
pub struct LogMessage {
    pub title: String,
    pub msg: String,
    pub msg_type: LogMessageType,
}

impl Default for LogMessages {
    fn default() -> Self {
        let mut log_file_path = "C:\\Users\\Public\\MechSim\\log\\".to_owned();
        log_file_path.push_str({
            let current_date = chrono::Local::now();
            format!("{}-{}-{}.txt", 
                current_date.day(),
                current_date.month(),
                current_date.year()
            ).as_str() 
        });

        match std::fs::create_dir_all("C:\\Users\\Public\\MechSim\\log") {
            Ok(()) => (),
            Err(e) => eprintln!("{e}")
        };

        Self {
            msgs: Default::default(),
            log_file: File::create(log_file_path).ok(),
        }
    }
}

impl LogMessages {
    fn push_msg(&mut self, title: &str, msg: &str, msg_type: LogMessageType) {
        self.msgs.push(LogMessage {
            title: title.to_owned(),
            msg: msg.to_owned(),
            msg_type: msg_type.clone(),
        });

        if let Some(file) = &mut self.log_file {
            let time = chrono::Local::now();
            write!(file, "{}:{}:{} [{}]: {}\n", time.hour(), time.minute(), time.second(), &msg_type, msg);
        }
    }

    pub fn info(&mut self, title: &str, msg: &str) {
        self.push_msg(title, msg, LogMessageType::Info);
    }

    pub fn warn(&mut self, title: &str, msg: &str) {
        self.push_msg(title, msg, LogMessageType::Warning);
    }

    pub fn error(&mut self, title: &str, msg: &str, popup: bool) {
        self.push_msg(title, msg, LogMessageType::Error { popup: popup });
    }
}

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LogMessages>();
    }
}