use ini::Ini;
use crate::start_ini::PathEmu;
use std::io;
use std::fs;
use std::path::Path;
use std::env;
pub mod start_ini;


pub fn get_config_path() -> Option<String>{
    let home = env::var("HOME").ok()?;
    Some(format!("{}/.config/emu-save/config.ini", home))
}

pub fn init_config(e_path : PathEmu){
    let mut conf = Ini::new();
    let path = get_config_path().expect("HOME not defined");

    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).expect("[ERROR]: Unable to create dir");
    }
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
    conf.write_to_file(&path).expect("[ERROR]: Not possible to write config file");
}
pub fn copy_dir(origin : &Path, dest : &Path) -> io::Result<()>{
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(origin)?{
        let entry = entry?;
        let path_origin = entry.path();
        let path_dest = dest.join(entry.file_name());

        if entry.file_type()?.is_dir(){
            copy_dir(&path_origin, &path_dest)?;
        }else if entry.file_type()?.is_file(){
            fs::copy(&path_origin, &path_dest)?;
        }
    }
    Ok(())
}
pub fn log_print(text:&str, is_ok:bool){
    if is_ok{
        println!("\x1b[32m[OK]\x1b[0m: {} ", text);
    }else{
        eprintln!("\x1b[31m[ERROR]\x1b[0m: {}", text);
    }
}
