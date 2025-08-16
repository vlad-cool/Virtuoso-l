use std::collections::VecDeque;
use std::vec::Vec;

/*
IP info / settings
Mirroring
IR info (current address)
Licenses
*/

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    Nop,
}

impl MenuAction {
    fn act(&self) {
        match self {
            Self::Nop => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Label(String),
    Button(String, MenuAction),
}

#[derive(Debug, Clone)]
pub struct SettingsMenu {
    show: bool,
    items: Vec<MenuItem>,
    index: usize,
    action_queue: VecDeque<MenuAction>,
}

impl SettingsMenu {
    pub fn is_shown(&self) -> bool {
        self.show
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn hide(&mut self) {
        self.show = false;
    }

    pub fn set_show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.items.len();
    }

    pub fn prev(&mut self) {
        self.index = (self.index + self.items.len() - 1) % self.items.len();
    }

    pub fn get_item(&self) -> &MenuItem {
        &self.items[self.index]
    }

    pub fn press(&mut self) {
        let item: &MenuItem = self.get_item();

        match item {
            MenuItem::Label(_) => {}
            MenuItem::Button(_, action) => {
                action.act();
            }
        }
    }
}
