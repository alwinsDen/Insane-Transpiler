use std::fs;
use std::env;
// use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
         let mut printed_js_data = String::from("//compiled with Insane-Compiler by github@alwinsDen;\n");
         for line_data in line_content.iter() {

             let mut root_element = String::new();

             //root element manipulation
             if line_data.starts_with("#") {
                 let proxy_data:Vec<&str> = line_data.split("#").collect();

                 root_element =  format!("let root=document.getElementById('{}')",proxy_data[1]);

                 printed_js_data += &root_element;
             }

             //text element creation
             else if line_data.contains(">>") {
                 let proxy_data:Vec<&str> = line_data.split_whitespace().collect();

                 let mut sub_root_write_element =String::new();

                 if proxy_data[0]=="p" || proxy_data[0]=="h1" || proxy_data[0]=="h2" {

                     sub_root_write_element = format!("document.createElement('{}')",&proxy_data[0]).to_string();

                     let mut proxy_text_data = String::new();

                     for text_data in 2..proxy_data.len() {

                         proxy_text_data += proxy_data[text_data];

                         proxy_text_data += " ";

                     }
                     printed_js_data += &format!("root.appendChild({}.appendChild(document.createTextNode({})))",&sub_root_write_element,&proxy_text_data);
                 }
             }
             printed_js_data+=";";
         }
        //build file creation
        let file_path = PathBuf::from(project_name.to_string()).join("build.js");
        let mut build_js_file = File::create(file_path).expect("ECMAscript build failed...");
        build_js_file.write_all(printed_js_data.as_bytes()).ok();

    }
    // println!("{:?}",folder_exist);
    // println!("{:?}",comp_project_path);
    "Project Compiled".to_string()
}