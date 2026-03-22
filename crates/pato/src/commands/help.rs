use crate::cli::HelpArgs;
use crate::topic_catalog::{find_topic, topic_names};
use std::io::{self, Write};
use std::process::ExitCode;

pub fn run(args: &HelpArgs) -> ExitCode {
    match args.topic.as_deref() {
        Some(topic) => match find_topic(topic) {
            Some(entry) => {
                print!("{}", entry.content);
                ExitCode::SUCCESS
            }
            None => {
                print_unknown_topic(topic);
                ExitCode::FAILURE
            }
        },
        None => {
            print_missing_topic();
            ExitCode::FAILURE
        }
    }
}

fn print_missing_topic() {
    let mut stderr = io::stderr().lock();
    let _ = writeln!(stderr, "usage: pato help <topic>");
    let _ = writeln!(stderr, "available topics:");
    for topic in topic_names() {
        let _ = writeln!(stderr, "  {topic}");
    }
}

fn print_unknown_topic(topic: &str) {
    let mut stderr = io::stderr().lock();
    let _ = writeln!(stderr, "error: unknown topic: {topic}");
    let _ = writeln!(stderr, "available topics:");
    for name in topic_names() {
        let _ = writeln!(stderr, "  {name}");
    }
}
