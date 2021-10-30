
use std::env;
use std::path::PathBuf;
use std::path::Path;

pub fn compile_project(project_name: &String)->PathBuf{
    let path_loc = env::current_dir().expect("Getting path failed");
    let folder_exist : bool =  Path::new(&path_loc).is_dir();
    println!("{:?}",folder_exist);
    path_loc
}