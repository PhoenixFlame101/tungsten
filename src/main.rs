use std::env::current_dir;
use std::fs;

use clap;
use clap::Arg;

fn init() -> Result<(), std::io::Error> {
    let dir_res = fs::create_dir(".tungsten");

    let cwd = current_dir().unwrap();
    let cwd = cwd.into_os_string().into_string().unwrap();

    match dir_res {
        Ok(()) => (),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Reinitialized existing Tungsten repository in {}", &cwd);
                return Ok(());
            } else {
                return Err(e);
            }
        }
    }

    println!("Initialized empty Tungsten repository in {}", &cwd);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // retrieve cli arguments
    let matches = clap::Command::new("Tungsten")
        .author("Abhinav Chennubhotla <abhinav@chennubhotla.com")
        .version("0.1.0")
        .about("Tungsten is an open-source, centralized version control system.")
        .arg(Arg::new("command").required(true))
        .get_matches();

    let exit_message = String::from("Invalid command.");
    let command = matches
        .get_one::<String>("command")
        .unwrap_or(&exit_message)
        .as_str();

    match command {
        "init" => init(),
        _ => Ok(println!("{exit_message}")),
    }
}
