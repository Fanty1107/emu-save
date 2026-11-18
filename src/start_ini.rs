use std::env;
use std::fmt;

#[derive(Debug)]
pub struct PathEmu {
    pub paths: Vec<(SupportedEmu, Option<String>)>,
    pub bak_path: Option<String>,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SupportedEmu {
    PCSX2,
    PPSSPP,
}

impl fmt::Display for SupportedEmu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedEmu::PCSX2 => write!(f, "PCSX2"),
            SupportedEmu::PPSSPP => write!(f, "ppsspp"),
        }
    }
}

impl Default for PathEmu {
    fn default() -> Self {
        Self {
            paths: vec![
                (SupportedEmu::PCSX2, get_default_path(SupportedEmu::PCSX2)),
                (SupportedEmu::PPSSPP, get_default_path(SupportedEmu::PPSSPP)),
            ],
            bak_path: get_default_bak(),
        }
    }
}
fn get_default_bak() -> Option<String> {
    let temp = env::temp_dir().join("emu-save-backup").into_string().expect("Unable to create default backup folder in tmp");
    Some(format!("{}", temp))
}

fn get_default_path(emu: SupportedEmu) -> Option<String> {
    let home = env::var("HOME").ok()?;
    match emu {
        SupportedEmu::PCSX2 => Some(format!("{}/.config/{}/memcards", home, emu)),
        SupportedEmu::PPSSPP => Some(format!("{}/.config/{}/PSP/SAVEDATA", home, emu)),
    }
}
