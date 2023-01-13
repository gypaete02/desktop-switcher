use crate::bspc::{self, get_desktops};

pub struct Desktops {
    desktops: usize,
    desktop_history: Vec<usize>,
}

impl Desktops {
    pub fn new() -> Self {
        let desktops = get_desktops().len();

        Self {
            desktop_history: vec![0],
            desktops,
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
            .position(|&d| d == current_index);

        match idx {
            Some(idx) => {
                let temp = self.desktop_history.remove(idx);
                self.desktop_history.insert(0, temp);
            }
            None => {
                self.desktop_history.insert(0, current_index)
            }
        }

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
            .position(|&d| d == current_index);

        match idx {
            Some(idx) => {
                let temp = self.desktop_history.remove(idx);
                self.desktop_history.insert(0, temp);
            }
            None => {
                self.desktop_history.insert(0, current_index)
            }
        }

        self.update();
    }

    /// Go to the specified desktop. E.g. to 5
    pub fn go_to(&mut self, index: usize) {
        let current_index = index.clamp(0, self.desktops);

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index);

        match idx {
            Some(idx) => {
                let temp = self.desktop_history.remove(idx);
                self.desktop_history.insert(0, temp);
            }
            None => {
                self.desktop_history.insert(0, current_index)
            }
        }

        self.update();
    }

    /// Go to the desktop that was last visited. If you were on 4, then 2 and then 8, and then called
    /// `last(2)` you will go to desktop 4
    pub fn last(&mut self, count: usize) {
        self.clean_history();

        let current_index = self.desktop_history[count % self.desktop_history.len()];

        let idx = self
            .desktop_history
            .iter()
            .position(|&d| d == current_index)
            .unwrap_or(0);

        let temp = self.desktop_history.remove(idx);
        self.desktop_history.insert(0, temp);

        self.update();
    }

    pub fn preview_last(&mut self, count: usize) {
        self.clean_history();

        let current_index = self.desktop_history[count % self.desktop_history.len()];
        bspc::go_to_tab(current_index + 1);
    }

    fn update(&self) {
        bspc::go_to_tab(self.desktop_history[0] + 1);
    }

    fn clean_history(&mut self) {
        let active_desktops = bspc::get_active_desktops();

        for i in 0..self.desktop_history.len().saturating_sub(1) {
            if !active_desktops.contains(&self.desktop_history[i]) {
                self.desktop_history.remove(i);
            }
        }
    }
}
