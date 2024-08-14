use std::{
    fs::{File, write, read_to_string},
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    env,
};
use home;


const HELP: &str = r#"Usage:
- todo add <object>
- todo done <number>
- todo list
Tip: You can run todo done ALL to remove all objects in the todo"#;



fn main() {
    //Todo: rename done to remove, make done cross out the objects (and save them in the file ofc)
    let args: Vec<String> = env::args().collect();
    let mut save_dir = home::home_dir().expect("No home found");
    save_dir = append_to_path(save_dir, "/.todo");

    if args.len() < 2 {
        println!("{}", HELP);
        return
    }


    match args[1].as_str() {
        "add" => {add(&save_dir, &args)},
        "done" => {done(&save_dir, &args)},
        "list" => {list(&save_dir)},
        _ => {println!("{}", HELP)}
    }
}

fn add(path: &PathBuf, text: &Vec<String>) {
    let mut full_text = String::new();
    for index in 2..text.len() {
        full_text += &text[index];
        full_text += " ";
    }
    
    let mut contents = match read_to_string(path) {
        Ok(file) => file,
        Err(_) => {write(path, "").expect("Dir not found (WRITE NEW FILE, yo ass fucked up fam this is code error type shit)"); String::new()}
    };

    if contents == "" {
        contents = full_text;
    } else {
        contents = format!("{}\n{}", contents.trim(), full_text);
    }
    

    let _ = write(path, contents);
    list(path);
}


fn done(path: &PathBuf, text: &Vec<String>) {
    if text.len() < 3 {
        list(path);
        return
    }
 
    if text[2] == "ALL" {
        let _ = write(path, "");
    }

    
    
    let mut contents = lines_from_file(path);
    if contents.is_empty() {
        list(path);
        return
    }

    let text_index: usize = match text[2].trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please input a number"); return}
    };
    if text_index > contents.len() {
        println!("No such index in the todo");
        return
    }

    contents.remove(text_index-1);



    let mut full_contents = String::new();
    for i in 0..contents.len() {
        full_contents += contents[i].as_str();
        full_contents += "\n";
    }
    
    
    let _ = write(path, full_contents.trim());
    list(path);
}   


fn list(path: &PathBuf) {
    let contents = lines_from_file(path);
    if contents.is_empty() {
        println!("\n\n .......empty");
        return
    }
    for index in 0..contents.len() {
        print!("{}. ", index + 1);
        println!("{}", contents[index]);
    }
}






fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("TODO file not found, please run todo add <object> first");
    let buf = BufReader::new(file);
    buf.lines()
         .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn append_to_path(p: PathBuf, s: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push(s);
    p.into()
}