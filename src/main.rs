use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;


fn create_project(filename:&String)-> std::io::Result<()> {
    fs::create_dir_all(filename.to_string()).expect("Folder creation failed");

    let file_path = PathBuf::from(filename.to_string()).join("index.html");

    let mut file = File::create(file_path).expect("File creation failed.");
    file.write_all(b"<!DOCTYPE html>\n<html lang=`en`>\n\
        <head>\n<meta charset=`UTF-8`>\n<meta http-equiv=`X-UA-Compatible` content=`IE=edge`>\n
        <meta name=`viewport` content=`width=device-width, initial-scale=1.0`>\n\
        <title>Document</title>\n</head><body>
        <div id=`root`></div>\n</body>\n</html>
    ").expect("File writing failed");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (&args[1]=="create" && args[2]!="") {
        create_project(&args[2]);
    }
}