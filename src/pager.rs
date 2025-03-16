use regex::Regex;
use std::io::prelude::*;
use std::io::IsTerminal;

const PAGER_COMMAND: &str = "less";

pub fn start_pager(output: &str) {
    let sanitized_output = sanitize_output(output);

    let mut less_child = std::process::Command::new(PAGER_COMMAND)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = less_child.stdin.take().unwrap();

    std::thread::spawn(move || stdin.write_all(sanitized_output.as_bytes()).unwrap());

    less_child.wait().unwrap();
}

fn sanitize_output(output: &str) -> String {
    if std::io::stdout().is_terminal() {
        output.to_string()
    } else {
        remove_colors(output)
    }
}

fn remove_colors(output: &str) -> String {
    let ansi_escape_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_escape_regex.replace_all(output, "").to_string()
}
