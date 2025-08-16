use std::vec::Vec;

/*
IP info / settings
Mirroring
IR info (current address)
Licenses
*/

#[derive(Debug, Clone)]
pub enum MenuItem {
    Label(String),
    Button(String),
}

#[derive(Debug, Clone)]
enum MenuElement {
    IpAddress,
}

impl MenuElement {
    pub fn to_item(&self) -> MenuItem {
        match self {
            Self::IpAddress => MenuItem::Label("sgf".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuTab {
    name: String,
    elements: Vec<MenuElement>,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct SettingsMenu {
    tabs: Vec<MenuTab>,
    index: usize,
}

impl SettingsMenu {
    pub fn new() -> Self {
        Self {
            tabs: vec![MenuTab {
                name: "Internet".to_string(),
                elements: vec![MenuElement::IpAddress],
                index: 0,
            }],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn prev(&mut self) {
        self.index = (self.index + self.tabs.len() - 1) % self.tabs.len();
    }

    pub fn get_item(&self) -> &MenuTab {
        &self.tabs[self.index]
    }
}
