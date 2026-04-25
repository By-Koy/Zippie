use std::env;
use std::process::Command;

mod conf;

struct File {
    path: String,
    name: String,
    ext: String,
    full: String,
}

fn main() {
    let file = parse_file_name(env::args().skip(1).collect());

    let command = conf::parse_conf_to_command(&file.full, &file.name, &file.ext, &file.path);

    let out = Command::new(&command.exec)
        .args(&command.args)
        .output();
    println!("{}\n{} {:?}\n{out:?}", file.name, command.exec, command.args);
}

fn parse_file_name(file: String) -> File {
    let full: String = file.clone();
    let name: String = file.split("/").last().unwrap().to_string();
    let ext: String = name.split(".").skip(1).collect::<Vec<&str>>().join(".");
    let path: String = file.replace(&format!("{name}"), "");

    println!("\n{full}\n{name}\n{ext}\n{path}");

    File{path, name, ext, full}
}
