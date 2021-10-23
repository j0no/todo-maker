use chrono::{DateTime, Local, NaiveDate};
use clap::{App, Arg};
use open;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static DEFAULT_DIR_NAME: &str = "Todos";

fn file_path(filename: &String) -> String {
    format!("{}/{}", format_dir_path(), filename)
}

fn write_file(filename: &String, content: String) {
    let filepath = file_path(&filename);
    let file_exists = Path::new(&filepath).exists();
    println!("{}", filename);
    if !file_exists {
        let mut file = match File::create(filepath) {
            Ok(file) => file,
            Err(err) => panic!("[{}]", err),
        };
        let file_write_resp = file.write_all(content.as_bytes());
        match file_write_resp {
            Ok(()) => (),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }
}
fn format_header(date_string: String) -> String {
    format!(
        "# Todo - {}
- [ ] sample todo

# Calorie Table

| Food Item               | Amount | Calories per unit | Calories      |
| ----------------------- | ------ | ----------------- | ------------- |
| water                   | 0      | 0                 | 0 calories    |
",
        date_string
    )
}
fn format_file_name(file_date_string: String) -> String {
    format!("Todo {}.md", file_date_string)
}
fn write_today_file() -> Option<String> {
    let today: DateTime<Local> = Local::now();
    let file_date_string = today.format("%m-%d-%y").to_string();
    let header_date_string = today.format("%B %d, %Y").to_string();
    let formatted_header = format_header(header_date_string);
    let filename = format_file_name(file_date_string);

    write_file(&filename, formatted_header);
    Some(filename)
}

fn write_date_file(formatted_date: &str) -> Option<String> {
    let parse_res = NaiveDate::parse_from_str(formatted_date, "%m/%d/%y");
    let date_from_string = match parse_res {
        Ok(date_) => date_,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let file_date_string = date_from_string.format("%m-%d-%y").to_string();
    let header_date_string = date_from_string.format("%B %d, %Y").to_string();
    let formatted_header = format_header(header_date_string);
    let filename = format_file_name(file_date_string);

    write_file(&filename, formatted_header);
    Some(filename)
}
fn get_home_dir() -> String {
    match env::home_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_string(),
            None => String::new(),
        },
        None => String::new(),
    }
}
fn format_dir_path() -> String {
    let home_dir = get_home_dir();
    format!("{}/{}", home_dir, DEFAULT_DIR_NAME)
}
fn open_with_editor(filename: &String, editor: String) {
    let filepath = file_path(&filename);
    match open::with(filepath, editor) {
        Ok(_exit_status) => (),
        Err(err) => println!("{}", err),
    }
}

fn mkdir_todo_dir_if_missing() {
    let dir_path = format_dir_path();
    let dir_path_exists = Path::new(&dir_path).exists();
    if !dir_path_exists {
        match fs::create_dir(dir_path) {
            Ok(()) => (),
            Err(err) => println!("{}", err),
        }
    }
}

fn main() {
    let matches = App::new("Todo Maker")
        .version("1.0")
        .author("j0no")
        .about("Make a new Todo List in Markup")
        .arg(
            Arg::new("DATE")
                .about("Set date of todo file (format: mm/dd/yy) ")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("EDITOR")
                .short('e')
                .long("editor")
                .value_name("EDITOR")
                .about("recieves editor name")
                .takes_value(true),
        )
        .get_matches();
    mkdir_todo_dir_if_missing();
    let filename: Option<String>;
    if let Some(date) = matches.value_of("DATE") {
        if date == "today" {
            filename = write_today_file();
        } else {
            filename = write_date_file(date);
        }
    } else {
        filename = write_today_file();
    }
    if let Some(editor) = matches.value_of("EDITOR") {
        match filename {
            Some(filename_string) => open_with_editor(&filename_string, editor.to_string()),
            None => (),
        }
    }
}
