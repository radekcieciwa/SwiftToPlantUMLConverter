extern crate serde_json;

use std::env;
use std::process::Command;
use serde_json::Value;

const CLASS_KEY: &str = "source.lang.swift.decl.class";
const ENUM_KEY: &str = "source.lang.swift.decl.enum";
const STRUCT_KEY: &str = "source.lang.swift.decl.struct";
const PROTOCOL_KEY: &str = "source.lang.swift.decl.protocol";

const VARIABLE_KEY: &str = "source.lang.swift.decl.var.instance";
const ENUM_ELEM_KEY: &str = "source.lang.swift.decl.enumelement";

const SUBSTRCTURE_KEY: &str = "key.substructure";
const INHERITED_TYPES_KEY: &str = "key.inheritedtypes";
const NAME_KEY: &str = "key.name";
const KIND_KEY: &str = "key.kind";

const REL_INHERIT: &str = "-up-|>";

enum TokenType {
    SClass(String),
    SStruct(String),
    SEnum(String),
    SProtocol(String),
    SVariable(String),
    SCase(String),
}

enum TokenItem {
    // items, parents
    Type(Vec<TokenType>, Vec<TokenType>),
    // item
    Item(TokenType),
}

fn evaluate_to_token(json: &Value) -> Option<TokenType> {
    let name = &json[NAME_KEY].as_str();
    let kind = &json[KIND_KEY].as_str();

    match (kind, name) {
        (Some(kind), Some(name)) => {
            let name = name.to_string();
            match kind {
                &CLASS_KEY => Some(TokenType::SClass(name)),
                &ENUM_KEY => Some(TokenType::SEnum(name)),
                &PROTOCOL_KEY => Some(TokenType::SProtocol(name)),
                &STRUCT_KEY => Some(TokenType::SStruct(name)),
                &VARIABLE_KEY => Some(TokenType::SVariable(name)),
                &ENUM_ELEM_KEY => Some(TokenType::SCase(name)),
                _ => None,
            }
        },
        (Some(_x), None) => None,
        (_, _) => None,
    }
}

// fn print_me(json: &Value) {
//     let name = &json["key.name"].as_str();
//     let kind = &json["key.kind"].as_str();
//
//     match (kind, name) {
//         (Some(CLASS_KEY), Some(name)) => println!("Class: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some("source.lang.swift.decl.enum"), Some(name)) => println!("Enum: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some("source.lang.swift.decl.enumelement"), Some(name)) => println!("Enum Case: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some("source.lang.swift.decl.struct"), Some(name)) => println!("Struct: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some("source.lang.swift.decl.protocol"), Some(name)) => println!("Protocol: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some("source.lang.swift.decl.var.instance"), Some(name)) => println!("Variable: {:?}\nmy kind: {}\n\n", name, kind.unwrap()),
//         (Some(_x), None) => { },
//         // (None, Some(x)) => println!("NO KIND\nmy name: {}\n\n", name.unwrap()),
//         (_, _) => println!("Please cover me: {:?}, {:?}\n\n", name, kind),
//     }
// }

fn evaluate_me(json: &Value) -> Vec<TokenType> {
    let option_token = evaluate_to_token(json);

    let inherited_types = &json[INHERITED_TYPES_KEY].as_array();
    if inherited_types.is_some() {
        let inherited_types_unwrapped = inherited_types.unwrap();
        if inherited_types_unwrapped.len() > 0 {
            for inner in inherited_types_unwrapped {
                println!("{} {} {}", name.unwrap(), REL_INHERIT, inner[NAME_KEY].as_str().unwrap());
            };
        };
    }
}

fn traverse_dependencies(json: &Value) {
    let value = evaluate_me(json);
    if value.is_some() {
        println!("{}", value.unwrap());
    }

    let inner_json: &Value = &json[SUBSTRCTURE_KEY];
    let inner_json_array = inner_json.as_array();

    if inner_json_array.is_some() {
        let inner_json_array_unwrapped = inner_json_array.unwrap();
        if inner_json_array_unwrapped.len() > 0 {
            for inner in inner_json_array_unwrapped {
                traverse_dependencies(inner);
            };
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let output = Command::new("sourcekitten")
        .arg("structure")
        .arg("--file")
        .arg(file_path)
        .output()
        .expect("failed to execute process");

    let class_json_string = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&class_json_string).expect("Nooooo!");
    traverse_dependencies(&json);
}
