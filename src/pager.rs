use std::io::prelude::*;

const PAGER_COMMAND: &str = "less";

pub fn start_pager(output: &str) {
    let mut less_child = std::process::Command::new(PAGER_COMMAND)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = less_child.stdin.take().unwrap();
    let data = output.to_string();

    std::thread::spawn(move || stdin.write_all(data.as_bytes()).unwrap());

    less_child.wait().unwrap();
}
