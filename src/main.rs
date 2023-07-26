use {
    std::{
        env,
        io,
        path::Path
    },
    sysinfo::{
        System, 
        SystemExt
    },
    ini::Ini,
    colored::Colorize,
};

fn main() {
    if is_game_started() {
        println!("{}! You have to close the game! Press Enter to close the window...", "ERROR".red());
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }
    
    let mut options_path : String = env::var("USERPROFILE").unwrap() + "/Documents/My Games/Binding of Isaac Repentance/options.ini";
    let mut args: Vec<_> = env::args().collect();
    if cfg!(target_os = "windows") {
        if args.len() > 1 {
            options_path = args.remove(1);
        }
    }
    else {
        if args.len() > 1 {
            options_path = args.remove(1);
        }
        else {
            println!("{}! You have to specify path to config! Press Enter to close the window...", "ERROR".red());
            io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    }

    if is_option_exist(&options_path) {
        println!("{}! Option file does not exists! Press Enter to close the window...", "ERROR".red());
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }
    
    toggle_console(&options_path);
    
    if is_consolse_enabled(&options_path) {
        println!("{} now! Press Enter to close the window...", "ENABLED".yellow());
    }
    else {
        println!("{} now! Press Enter to close the window...", "DISABLED".green());
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn is_consolse_enabled(options_path : &String) -> bool {
    let mut conf = Ini::load_from_file(&options_path).unwrap();
    match conf.with_section(Some("Options")).get("EnableDebugConsole").unwrap() {
        "1" => return true,
        _ => return false,
    };
}

fn is_game_started() -> bool {
    let system = System::new_all();
    if system.processes_by_exact_name("isaac-ng.exe").count() > 0 {
        return true
    }
    else {
        return false
    }
}

fn is_option_exist(options_path : &String) -> bool {
    if !Path::new(&options_path).exists() {
        return true;
    }
    else {
        return false;
    }
}

fn toggle_console(options_path : &String) {
    let mut conf = Ini::load_from_file(&options_path).unwrap();
    match conf.with_section(Some("Options")).get("EnableDebugConsole").unwrap() {
        "1" => conf.with_section(Some("Options")).set("EnableDebugConsole", "0"),
        _ => conf.with_section(Some("Options")).set("EnableDebugConsole", "1"),
    };
    conf.write_to_file(&options_path).unwrap();
}
