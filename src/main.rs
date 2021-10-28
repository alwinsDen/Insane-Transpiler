use ansi_term::Colour;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn create_project(filename: &String) -> std::io::Result<()> {
    fs::create_dir_all(filename.to_string()).expect("Folder creation failed");
    fs::create_dir_all(filename.to_string() + "/" + "src")
        .expect(&Colour::Red.paint("Sub-Folder creation failed.").to_string());

    File::create(PathBuf::from(filename.to_string() + "/" + "src").join("index.ic")).ok();
    let file_path = PathBuf::from(filename.to_string()).join("index.html");

    let mut file = File::create(file_path).expect("File creation failed.");
    let mut file_css = File::create(PathBuf::from(filename.to_string()).join("style.css")).expect("CSS creation failed");
    file.write_all(b"<!DOCTYPE html>\n<html lang=`en`>\n<head>\n<meta charset=`UTF-8`>\n<meta http-equiv=`X-UA-Compatible` content=`IE=edge`>\n<meta name=`viewport` content=`width=device-width, initial-scale=1.0`>\n<title>Document</title>\n<link rel='stylesheet' href='style.css'>\n</head><body><div id='root'></div>\n</body>\n</html>\n<!--Complied with Insane-Compliter 0.0.1 2021-->").expect("File writing failed");
    file_css.write_all(b"/*Generated with insane-compliter for web development*/").expect("Writing to CSS failed");

    println!("{}",Colour::Green.paint("Filed created sucessfully. Weeee!!!").to_string());
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args[1] == "create" && args[2] != "" {
        create_project(&args[2]).ok();
    }
}
