use ini::Ini;
use std::io::Error;
mod start_ini;
use crate::start_ini::PathEmu;
fn main() -> Result<(), Error> {
    let emu_paths = PathEmu::default();
    let mut conf = Ini::new();
    conf.with_section(None::<String>).set("encoding", "utf-8");
    let mut section = conf.with_section(Some("Saves"));


    for (emu, path) in &emu_paths.paths {
        if let Some(p) = path {
            section.set(emu.to_string(), p);
        }
    }
    let mut bak_section = conf.with_section(Some("Backup"));
    if let Some(b) = emu_paths.bak_path{
        bak_section.set("bak_path", b);
    }
    conf.write_to_file("config.ini").unwrap();
    Ok(())
}
