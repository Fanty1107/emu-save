use ini::Ini;
use std::fs;
use std::io;
use autosave::{CONFIG_PATH, init_config};
use autosave::start_ini::PathEmu;

fn main() -> io::Result<()> {
    let emu_paths = PathEmu::default();
    match fs::exists("config.ini"){
        Ok(true) => println!("[LOG]: Config file exists"),
        Ok(false) => init_config(emu_paths),
        Err(e) => eprintln!("[ERROR]: permission denied or I/O error: {e}"),
    }
    let conf = Ini::load_from_file(CONFIG_PATH).unwrap();
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
           None => eprintln!("encoding=utf-8"),
           _ => eprintln!("[ERROR]: undefined"),
        };
    }
    for (key, value) in saves_path.iter(){
        println!("{}:{}", key,value); 
    }
    println!("{}", bak_path);

    Ok(())
}
