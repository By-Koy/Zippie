use std::env;
use std::process::Command;


struct File {
    Name: String,
    Ext: String
}

fn main() {
    let file = parse_file(env::args().skip(1).collect());
    let out = Command::new("unzip")
        .arg(format!("{}{}", file.Name, file.Ext))
        .output();
    println!("{}{}", file.Name, file.Ext);
    
}

fn parse_file(file: String) -> File {
    let name: &str = file.split("/").last().unwrap().split(".").next().unwrap();
    let ext: String = String::from(".") + &file.split(".").skip(1).collect::<Vec<&str>>().join(".");
    File{Name: name.to_string(), Ext: ext}
}
