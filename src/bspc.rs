use std::process::Command;

/// Returns a Vec containing the IDs of the desktops.
pub fn get_desktops() -> Vec<usize> {
    let bytes = Command::new("/usr/bin/bspc")
        .args(["query", "-D"])
        .output()
        .expect("Error: could not execute bspc")
        .stdout;

    String::from_utf8_lossy(&bytes)
        .lines()
        .map(|line| line.trim_start_matches("0x"))
        .filter_map(|line| i64::from_str_radix(line, 16).ok())
        .map(|n| n as usize)
        .collect()
}

pub fn go_to_tab(desktop_id: usize) {
    Command::new("/usr/bin/bspc")
        .args(["desktop", "-f", format!("^{desktop_id}").as_str()])
        .spawn()
        .expect("Error: could not execute bspc");
}

/// Returns a Vec containing the relative index of the desktops that are active.
pub fn get_active_desktops() -> Vec<usize> {

    let desktops = get_desktops();

    let bytes = Command::new("/usr/bin/bspc")
        .args(["query", "-D", "-d", ".occupied"])
        .output()
        .expect("Error: could not execute bspc")
        .stdout;

    String::from_utf8_lossy(&bytes)
        .lines()
        .map(|line| line.trim_start_matches("0x"))
        .filter_map(|line| i64::from_str_radix(line, 16).ok())
        .map(|n| to_relative(&desktops, n as usize))
        .collect()
}

fn to_relative(desktops: &Vec<usize>, n: usize) -> usize {
    for (i, desktop) in desktops.iter().enumerate() {
        if n == *desktop {
            return i;
        }
    }
    return 0;
}
