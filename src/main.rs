mod input;
mod bspc;
mod desktops;

fn main() {

    if std::env::args().len() > 1 {
        println!("{}", HELP);
        return;
    }

    input::start_listening().unwrap();
}

const HELP: &'static str = 
"
desktop-switcher

desktop-switcher is program for switching between bspwm desktops conveniently with alt + tab.

    Switch between recently used desktops with alt + tab
    Switch to specific desktop with super + 0..9
    Switch to next/previous desktop with super + [ / super + ]

    source code at https://github.com/gypaete02/desktop-switcher
";
