use crate::bspc::{self, get_desktop_count};

pub struct Desktops {
    desktops: usize,
    current_index: usize,
    desktop_history: Vec<usize>,
}

impl Desktops {
    pub fn new() -> Self {
        let desktops = get_desktop_count();

        Self {
            desktop_history: (0..desktops).collect(),
            desktops,
            current_index: 0,
        }
    }

    /// Go to the next desktop. E.g. from 5 to 6
    pub fn next(&mut self) {
        let mut current_index = self.desktop_history[0] + 1;

        if current_index >= self.desktops {
            current_index = 0;
        }

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index)
            .unwrap_or(0);

        let temp = self.desktop_history.remove(idx);
        self.desktop_history.insert(0, temp);

        self.update();
    }

    /// Go to the previous desktop. E.g. from 5 to 4
    pub fn previous(&mut self) {
        let current_index = if self.desktop_history[0] == 0 {
            self.desktops.saturating_sub(1)
        } else {
            self.desktop_history[0] - 1
        };

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index)
            .unwrap_or(0);

        let temp = self.desktop_history.remove(idx);
        self.desktop_history.insert(0, temp);

        self.update();
    }

    /// Go to the specified desktop. E.g. to 5
    pub fn go_to(&mut self, index: usize) {
        let current_index = index.clamp(0, self.desktops);

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index)
            .unwrap_or(0);

        let temp = self.desktop_history.remove(idx);
        self.desktop_history.insert(0, temp);

        self.update();
    }

    /// Go to the desktop that was last visited. If you were on 4, then 2 and then 8, and then called
    /// `last(2)` you will go to desktop 4
    pub fn last(&mut self, count: usize) {
        let current_index = self.desktop_history[count % self.desktops];

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index)
            .unwrap_or(0);

        let temp = self.desktop_history.remove(idx);
        self.desktop_history.insert(0, temp);

        self.update();
    }

    pub fn preview_last(&self, count: usize) {
        let current_index = self.desktop_history[count % self.desktops];
        bspc::go_to_tab(current_index + 1);
    }

    fn update(&self) {
        bspc::go_to_tab(self.desktop_history[0] + 1);
    }
}
