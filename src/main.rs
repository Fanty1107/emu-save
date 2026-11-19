use ini::Ini;
use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};
use autosave::{init_config, copy_dir, get_config_path, log_print};
use autosave::start_ini::PathEmu;

fn main() -> io::Result<()> {
    let emu_paths = PathEmu::default();
    let config_path = get_config_path().expect("Home not defined");
    let is_ok = true;
        match fs::exists(&config_path){
        Ok(true) => log_print(&format!("Config file path: {}", config_path), is_ok),
        Ok(false) => init_config(emu_paths),
        Err(e) => log_print(&format!("permission denied or I/O error: {e}"), !is_ok),
    }
    let conf = Ini::load_from_file(&config_path).unwrap();
    let mut saves_path : Vec<(&str, &str)> = Vec::new();
    let mut bak_path = String::new();
    for (sec, prop) in &conf{
        match sec{
           Some("Backup") => {
                for(_, value) in prop.iter(){
                    bak_path.push_str(value);
                }    
           },
           Some("Saves") => {
                for(key, value) in prop.iter(){
                    saves_path.push((key,value));
                }
           },
           None => (),
           _ => log_print("check the config.ini file struct", !is_ok),
        };
    }
    let mut origins :Vec<PathBuf> = Vec::new();
    for (_, o_paths) in saves_path{
        origins.push(PathBuf::from(o_paths));
    }
    let dest = Path::new(&bak_path);

    for orig in &origins{
        let name_dir = orig.file_name().ok_or_else(||{
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid name")
        })?;
        copy_dir(orig, &dest.join(name_dir))?;
    }
    log_print(&format!("SUCESS check: {}", bak_path), is_ok);
    Ok(())
}
