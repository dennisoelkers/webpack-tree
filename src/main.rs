extern crate core;

use serde::{Deserialize};
use std::fs::File;
use std::io::prelude::*;
use colored::*;
use std::process;

#[derive(Deserialize,Debug)]
#[allow(non_snake_case)]
struct Reason {
    moduleId: Option<String>,
    userRequest: Option<String>,
}

#[derive(Deserialize,Debug)]
struct Module {
    id: String,
    identifier: String,
    name: String,
    reasons: Vec<Reason>,
}

#[derive(Deserialize,Debug)]
struct BuildResult {
    modules: Vec<Module>,
}

fn find_module_by_id(modules: &Vec<Module>, id: String) -> Option<Module> {
    modules.into_iter().find(|module| { module.id == id }).map(|module| module.clone())
}

impl std::clone::Clone for Reason {
    fn clone(&self) -> Self {
        Reason { moduleId: self.moduleId.clone(), userRequest: self.userRequest.clone() }
    }
}
impl std::clone::Clone for Module {
    fn clone(&self) -> Self {
        Module { id: self.id.clone(), identifier: self.identifier.clone(), name: self.name.clone(), reasons: self.reasons.clone() }
    }
}
impl std::clone::Clone for BuildResult {
    fn clone(&self) -> Self {
        BuildResult { modules: self.modules.clone() }
    }
}

fn traverse_reasons(root: &BuildResult, module: Module, visited: &mut Vec<String>, level: usize) {
    if level == 0 {
        println!("{} ({})", module.name.green(), module.id.bold().green());
    } else {
        println!("{:level$}- {} ({})", "", module.name.green(), module.id.bold().green(), level = level);
    }
    if visited.contains(&module.id) {
        println!("{:level$} {}", "", "<Circular dependency>".red(), level = (level + 2));
        return ();
    }
    visited.push(module.id.clone());
    module.reasons.into_iter().for_each(|reason| {
        match reason.moduleId {
            Some(id) => match find_module_by_id(&root.modules, id) {
                Some(reason_module) => traverse_reasons(root, reason_module, &mut visited.clone(),level + 2),
                None => println!("{:level$}- {}", "", "<End>".blue(), level = (level + 2)),
            },
            None => println!("{:level$}- {}", "", "<End>".blue(), level = (level + 2)),
        }
    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Syntax: {} <filename> <module id>", args[0]);
        process::exit(-127);
    }

    let filename = args[1].to_string();
    let mut file = File::open(filename).unwrap();

    let module_id = args[2].to_string();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let build_result: BuildResult = serde_json::from_str(contents.as_str()).unwrap();

    let entry = find_module_by_id(&build_result.modules, module_id);
    let mut visited: Vec<String> = Vec::new();
    traverse_reasons(&build_result, entry.unwrap(), &mut visited, 0);
}
