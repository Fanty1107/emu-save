use ini::Ini;
use crate::start_ini::PathEmu;
pub mod start_ini;

pub const CONFIG_PATH : &str = "config.ini";

pub fn init_config(e_path : PathEmu){
    let mut conf = Ini::new();
    conf.with_section(None::<String>).set("encoding", "utf-8");
    let mut section = conf.with_section(Some("Saves"));

    for (emu, path) in &e_path.paths {
        if let Some(p) = path {
            section.set(emu.to_string(), p);
        }
    }
    let mut bak_section = conf.with_section(Some("Backup"));
    if let Some(b) = e_path.bak_path{
        bak_section.set("bak_path", b);
    }
    conf.write_to_file("config.ini").unwrap();
}
