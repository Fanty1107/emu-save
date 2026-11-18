use ini::Ini;
use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};
use autosave::{init_config, copy_dir, get_config_path};
use autosave::start_ini::PathEmu;

fn main() -> io::Result<()> {
    let emu_paths = PathEmu::default();
    let config_path = get_config_path().expect("Home not defined");
    match fs::exists(&config_path){
        Ok(true) => println!("[LOG]: Config file exists"),
        Ok(false) => init_config(emu_paths),
        Err(e) => eprintln!("[ERROR]: permission denied or I/O error: {e}"),
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
           _ => eprintln!("[ERROR]: undefined"),
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
    println!("[OK]: SUCESS check {}", bak_path);
    Ok(())
}
