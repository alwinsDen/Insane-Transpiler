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

         let mut element_serializer = 0;

         // let mut current_element_var = String::new();

        'ic_mapper: for line_data in line_content.iter() {
             if line_data.starts_with("//") {
                 continue 'ic_mapper;
             }
             let mut root_element = String::new();
             //root element manipulation
             if line_data.starts_with("#") {
                 let proxy_data:Vec<&str> = line_data.split("#").collect();

                 root_element =  format!("let {}=document.getElementById('{}')",proxy_data[1],proxy_data[1]);
                 // current_element_var = proxy_data[1].to_string();

                 printed_js_data += &root_element;
             }

             //text element creation
             else if line_data.contains(">>") {
                 let proxy_data:Vec<&str> = line_data.split_whitespace().collect();

                 let _sub_root_write_element =String::new();

                 if proxy_data[0]=="p" || proxy_data[0]=="h1" || proxy_data[0]=="h2" || proxy_data[0]=="h3" ||proxy_data[0]=="h4" ||proxy_data[0]=="h5"{

                     printed_js_data += &format!("let writeElement{} = document.createElement('{}');",&element_serializer,&proxy_data[0]);

                     let mut proxy_text_data = String::new();

                     'text_looper : for text_data in 2..proxy_data.len() {
                         if proxy_data[text_data] == "||" {
                             let custom_id_tag = String::from(proxy_data[text_data+1]);
                             if custom_id_tag!="" {
                                 printed_js_data +=  &format!("writeElement{}.id={};",&element_serializer,&custom_id_tag);
                                 break 'text_looper;
                             }
                         };
                         proxy_text_data += proxy_data[text_data];

                         proxy_text_data += " ";

                     }
                     printed_js_data += &format!("writeElement{}.appendChild(document.createTextNode({}));",&element_serializer,&proxy_text_data);
                     printed_js_data += &format!("root.appendChild(writeElement{})",&element_serializer);
                 }
                 else if proxy_data[0]=="div" {
                     printed_js_data += &format!("let divElement{}=document.createElement('{}');",&element_serializer,&proxy_data[0]);
                 }
                     //buttons and other input elements
                 else if proxy_data[0]=="button" {
                     printed_js_data += &format!("let inputElement{} = document.createElement('{}');",&element_serializer,&proxy_data[0]);
                     let mut proxy_text_data = String::new();
                     'input_looper : for text_data in 2..proxy_data.len() {
                         if proxy_data[text_data] == "||" {
                             let custom_id_tag = String::from(proxy_data[text_data+1]);
                             if custom_id_tag!="" {
                                 printed_js_data +=  &format!("inputElement{}.id={};",&element_serializer,&custom_id_tag);
                                 break 'input_looper;
                             }
                         };
                         proxy_text_data += proxy_data[text_data];

                         proxy_text_data += " ";
                     }
                     printed_js_data += &format!("inputElement{}.appendChild(document.createTextNode({}));",&element_serializer,&proxy_text_data);
                     printed_js_data += &format!("root.appendChild(inputElement{})",&element_serializer);
                 }

                 else if proxy_data[0]=="style" {
                     if proxy_data[2]=="default"{
                         printed_js_data += &format!("\
                         let allElements=document.querySelectorAll('*');\
                         for (let i=0;i<allElements.length;i++){{\
                         allElements[i].style.margin='0';\
                         allElements[i].style.padding='0';\
                         allElements[i].style.boxSizing='border-box'}}");
                     }
                     else if proxy_data[2]=="custom" && proxy_data[3]!="" {
                         let mut calc_bound = true;
                         'style_loop:for data_range in 5..proxy_data.len() {
                             if proxy_data[data_range]=="->" {calc_bound=true;continue 'style_loop;}
                             else if proxy_data[data_range]==":" || calc_bound ==false {
                                 continue 'style_loop;
                             }
                             if calc_bound {
                                 printed_js_data += &format!("document.getElementById({}).style.{}='{}';",proxy_data[3],proxy_data[data_range],proxy_data[data_range+2]);
                                 calc_bound = false;
                             }
                         }
                         // println!("{}",&& proxy_data[3]);
                     }
                 }
             }

            //this part deals with functions
            else if line_data.contains("()") {
                let proxy_data:Vec<&str> = line_data.split_whitespace().collect();
                let mut sub_function = String::new();
                //sub functions
                if proxy_data[4]=="alert" {
                    let mut alert_text = String::new();
                    'subfunction : for alert_index in 6..proxy_data.len() {
                        if proxy_data[alert_index]=="<-()" {break 'subfunction};
                        alert_text+= proxy_data[alert_index];
                        alert_text+=" ";
                    }
                    sub_function = format!("window.alert({})",&alert_text);
                }

                //main functions
                if proxy_data[0]=="click" || proxy_data[0]=="mouseenter" || proxy_data[0]=="mouseleave" && proxy_data[4]!=""{
                    printed_js_data += &format!("document.getElementById({}).addEventListener('{}',()=>{});",&proxy_data[2],&proxy_data[0],&sub_function);
                    if proxy_data[proxy_data.len()-2]=="<-()" || proxy_data[proxy_data.len()-1]=="<-()"{
                        printed_js_data += &format!("document.getElementById({}).removeEventListener('{}',()=>{});",&proxy_data[2],&proxy_data[0],&sub_function);
                    }
                    if proxy_data[proxy_data.len()-1]=="LOGGER" {
                        printed_js_data += &format!("document.getElementById({}).addEventListener('{}',()=>console.log({}+' was clicked'));",&proxy_data[2],&proxy_data[0],&proxy_data[2]);
                    }
                }
            }
            if line_data!=&"" {
                printed_js_data+=";";
            }
             element_serializer += 1;
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