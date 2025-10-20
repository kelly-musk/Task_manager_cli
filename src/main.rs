use clap::{Command, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use core::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const FILE_PATH: &str = "task.json";
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    done: bool
}

#[derive(Parser)] 
#[command(name = "Task Manager")]
#[command(about = "A task manager that to track your daily life progress")] 
struct Cli {
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {title: String},
    List,
    Done { index: usize },
    Delete { index: i32}
}
fn main() {
    println!{"he"}
}

