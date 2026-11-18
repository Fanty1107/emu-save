use ini::Ini;
use crate::start_ini::PathEmu;
pub mod start_ini;
use std::fs;
use std::io;

fn init_config(e_path : PathEmu){
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

fn main() -> io::Result<()> {
    let emu_paths = PathEmu::default();
    match fs::exists("config.ini"){
        Ok(true) => println!("[LOG]: Config file exists"),
        Ok(false) => init_config(emu_paths),
        Err(e) => eprintln!("[ERROR]: permission denied or I/O error: {e}"),
    }
    Ok(())
}
