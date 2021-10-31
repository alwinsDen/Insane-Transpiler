use std::fs;
use std::env;
// use std::path::PathBuf;
use std::path::Path;

pub fn compile_project(project_name: &String)->String{
    let path_loc = env::current_dir().expect("Getting path failed");
    let mut comp_project_path = String::new();
    comp_project_path +=   &path_loc.into_os_string().into_string().unwrap().to_string();
    comp_project_path += "/";
    comp_project_path +=      &project_name.to_string();
    comp_project_path += "/src";
    let folder_exist : bool =  Path::new(&comp_project_path).is_dir();
    if folder_exist==true {
         let contents = fs::read_to_string(format!("{}{}",&comp_project_path,"/index.ic")).expect("Error reading the file.");
        let line_content:Vec<&str> = contents.split("\n").collect();
         println!("{:?}",line_content);
    }
    println!("{:?}",folder_exist);
    println!("{:?}",comp_project_path);
    "Project Compiled".to_string()
}