use std::fs;
use std::io::Write;
use std::io::Error;
use std::path::Path;
use toml::Table;
use std::collections::HashMap;

pub struct Command {
    pub exec: String,
    pub args: Vec<String>
}

fn make_conf() -> Result<(), Error> {
    let mut file=fs::File::create("zippie.toml")?;
    file.write(b"Lorem a posum!")?;
    println!("Created a new config file, please update as needed or check [PLACEHOLDER] for an updated config");
    Ok(())
}

pub fn read_conf_to_string() -> Result<String, Error> {
    let conf_file = fs::read_to_string(Path::new("zippie.toml"));
    if let Ok(_) = conf_file {
        println!("config file has been found!");
        Ok(conf_file.unwrap())
    } else {
        make_conf()?;
        println!("config file not found! \ncreating a new one and retrying.");
        conf_file
    }
}

pub fn parse_conf_to_command(input:&str, name:&str, ext: &str, path: &str) -> Command {
    let conf_file: String;
    match read_conf_to_string() {
        Ok(conf) => conf_file=conf,
        Err(_) => conf_file=read_conf_to_string().unwrap(),
    };

    let filename = name.replace(&format!(".{ext}"),"");
    let mut conf_map = HashMap::new();
    let exec: String;
    let args: Vec<String>;

    for table in &mut conf_file.split_inclusive("\n\n") {
        conf_map.insert(table.split(|x| x=='[' || x==']').skip(1).next().unwrap()
            .to_string(), table.parse::<Table>().unwrap());
    }
    println!("\n{conf_map:?}");

    if let Some(_) = conf_map.get(ext) {
        let command = conf_map.get(ext).expect("{ext} does not seem to be defined in the config file.\n see [PLACEHOLDER for more info]")
            .get(ext).unwrap().get("extract")
            .expect("extract command for {ext} is not provided.\nsee [PLACEHOLDER] for more info.")
            .to_string().replace("@O", &format!("{path}{filename}")).replace("@I", &format!("{input}"));

        exec = command.split(" ").next().unwrap().to_string();
        args = command.split(" ").skip(1).map(|x| x.to_string()).collect();
    } else {
        let command = conf_map.get("zip").expect("support for zip files is currently required to archive a file. for more info see [Placeholder]")
            .get("zip").unwrap().get("archive")
            .expect("unable to get command to archive a zip file\nsee [PLACEHOLDER] for more info")
            .to_string().replace("@O", &format!("{path}{filename}.zip")).replace("@I", &format!("{input}"));

        println!("\nCommand: {command}\n");

        exec = command.split(" ").next().unwrap().to_string();
        args = command.split(" ").skip(1).map(|x| x.to_string()).collect();
    }

    Command{exec: exec.clone(), args: args.clone()}
}