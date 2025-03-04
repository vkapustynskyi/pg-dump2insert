use std::io;
use std::io::prelude::*;
extern crate regex;
use std::fs::File;
use regex::Regex;

fn main() {
    let mut file = File::create("converted.sql").expect("Error encountered while creating file!");
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    let mut table_name = String::new();
    let mut fields = String::new();
    let re = Regex::new(r"^COPY (.+) \((.+)\) FROM stdin;").unwrap();
    let mut insert_mode = false;
    while stdin.read_line(&mut line).unwrap() > 0 {
        if insert_mode {
            if line == "\\.\n" {
                insert_mode = false;
            } else {
                let mut values = String::new();
                line.pop();
                for s in line.replace("'", "''").split("\t") {
                    if s == "\\N" {
                        values += "NULL, ";
                    } else {
                        values += "'";
                        values += s;
                        values += "', ";
                    }
                }
                values.pop();
                values.pop();
                file.write(format!("INSERT INTO {} ({}) VALUES ({});\n", table_name, fields, values).to_owned().as_bytes());
            }
        } else {
            match re.captures(&line) {
                None => print!("{}", line),
                Some(caps) => {
                    table_name = String::from(caps.get(1).map_or("", |m| m.as_str()));
                    fields = String::from(caps.get(2).map_or("", |m| m.as_str()));
                    insert_mode = true;
                }
            };
        }
        line.clear();
    }
}
