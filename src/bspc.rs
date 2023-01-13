use std::{process::Command, io::read_to_string};

pub fn get_desktop_count() -> usize {
    let bytes = Command::new("/usr/bin/bspc")
        .args(["query", "-D"])
        .output()
        .expect("Error: could not execute bspc")
        .stdout;
    
    String::from_utf8_lossy(&bytes).to_string()
        .lines()
        .count()
}

pub fn go_to_tab(desktop_id: usize) {
    Command::new("/usr/bin/bspc")
        .args(["desktop", "-f", format!("^{desktop_id}").as_str()])
        .spawn()
        .expect("Error: could not execute bspc");
}
