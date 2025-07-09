mod menu;
use std::io;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::string::String;
use git2::Repository;
use std::fs::File;
use std::io::{Write};
extern crate dirs;

fn clone_repo(repo_url: &str, target_dir: &str) -> Result<(), git2::Error> {
    let path = Path::new(target_dir);
    Repository::clone(repo_url, path)?;
    Ok(())
}

fn create_config(text: &str) -> io::Result<()> {
    let mut path = dirs::config_dir().unwrap(); 
    path.push("config.stark");
   
    let mut file = File::create(&path)?;

    writeln!(file,"{}", text)?;
    
    Ok(()) 
}

fn install()
{

    println!("##############################"); 
    println!("#        START INSTALL       #");
    println!("##############################");

    println!("Installation path:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
        
    let dir_name = input.trim();
    let path: PathBuf = PathBuf::from(dir_name); 

    match fs::create_dir_all(&path) {
        Ok(_) => println!("Directory created: {:?}", path),
        Err(e) => {
            eprintln!("Error: Create dir: {}", e);
            return; 
        }
    }    

    let binding: &str = &path.to_string_lossy();
    let path_str: &str = binding.as_ref();
    let repo_url = "https://github.com/Saka1r/Jarvis.git"; 

    match clone_repo(repo_url, path_str) {
        Ok(_) => println!("Nice, package installed"),
        Err(e) => eprintln!("Error: {}", e),
    }
    create_config(&path_str);
    
    println!("##############################"); 
    println!("#        CREATED CONFIG      #");
    println!("##############################");
    let mut path = dirs::config_dir().unwrap(); 
    println!("path to config -> {:?}", path);
}

fn run_app()
{

}

fn update(){}

fn reinstall(){}

fn remove(){}

fn main() -> io::Result<()> {
    menu::menu_list();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    match input.trim_end() {
    "Run" => run_app(),
    "Install" => install(),
    "Update" => update(),
    "Reinstall" => reinstall(),
    "Remove" => remove(),
    _ => panic!("Error: Non corrent input"),
    }

    Ok(())
}
