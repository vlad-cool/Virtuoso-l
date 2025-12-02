use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::{
    sync::{Arc, atomic::AtomicBool},
    vec::Vec,
};

use self_update::cargo_crate_version;

use crate::modules::{HardwareConfig, Logger};

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
pub enum MenuElement {
    IpAddressWln,
    IpAddressEth,
    UpdateBtn(String),
    ExitBtn,
}

impl MenuElement {
    pub fn to_item(&self) -> MenuItem {
        match self {
            Self::IpAddressEth => {
                let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();
                let interface: Option<&datalink::NetworkInterface> =
                    interfaces.iter().find(|item| item.name == "end0");

                if let Some(interface) = interface {
                    let ipv4: Option<&IpNetwork> = interface
                        .ips
                        .iter()
                        .find(|item: &&IpNetwork| item.is_ipv4());
                    let ipv4: String = if let Some(ipv4) = ipv4 {
                        ipv4.ip().to_string()
                    } else {
                        "-".to_string()
                    };

                    // let ipv6: Option<&IpNetwork> = interface.ips.iter().find(|item| item.is_ipv6());
                    // let ipv6: String = if let Some(ipv6) = ipv6 {
                    //     ipv6.ip().to_string()
                    // } else {
                    //     "-".to_string()
                    // };

                    MenuItem::Label(format!("Ethernet\n{}", ipv4))
                } else {
                    MenuItem::Label(format!("No\ninterface\nfound"))
                }
            }
            Self::IpAddressWln => {
                let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();
                let interface: Option<&datalink::NetworkInterface> =
                    interfaces.iter().find(|item| item.name == "wlan0");

                if let Some(interface) = interface {
                    let ipv4: Option<&IpNetwork> = interface
                        .ips
                        .iter()
                        .find(|item: &&IpNetwork| item.is_ipv4());
                    let ipv4: String = if let Some(ipv4) = ipv4 {
                        ipv4.ip().to_string()
                    } else {
                        "-".to_string()
                    };

                    MenuItem::Label(format!("Wi-Fi\n{}", ipv4))
                } else {
                    MenuItem::Label(format!("No\ninterface\nfound"))
                }
            }
            Self::UpdateBtn(status) => MenuItem::Button(format!("Update\n{status}")),
            Self::ExitBtn => MenuItem::Button(format!("Exit")),
        }
    }

    pub fn press(&mut self, logger: &Logger, menu_shown: Arc<AtomicBool>) {
        match self {
            Self::UpdateBtn(res_status) => {
                let mut backend: self_update::backends::github::UpdateBuilder =
                    self_update::backends::github::Update::configure();

                let update_builder: &mut self_update::backends::github::UpdateBuilder = backend
                    .repo_owner("vlad-cool")
                    .repo_name("Virtuoso-l")
                    .bin_name("Virtuoso")
                    .no_confirm(true)
                    .show_download_progress(false)
                    .current_version(cargo_crate_version!());

                let update = match update_builder.build() {
                    Ok(update) => update,
                    Err(err) => {
                        logger.error(format!("Failed to build updater, err: {err}"));
                        *res_status = "error".into();
                        return;
                    }
                };

                let status: self_update::Status = match update.update() {
                    Ok(status) => status,
                    Err(err) => {
                        logger.error(format!("Failed to update, err: {err}"));
                        *res_status = "error".into();
                        return;
                    }
                };

                if status.updated() {
                    *res_status = format!("Successfully\nupdated to\n{}", status.version());
                } else {
                    *res_status = format!("Up to date");
                }
            }
            Self::ExitBtn => {
                menu_shown.store(false, std::sync::atomic::Ordering::Relaxed);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuTab {
    name: String,
    elements: Vec<MenuElement>,
    index: usize,
}

impl MenuTab {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_elements(&self) -> &Vec<MenuElement> {
        &self.elements
    }

    #[allow(dead_code)]
    pub fn get_active(&self) -> &MenuElement {
        &self.elements[self.index]
    }

    pub fn get_active_mut(&mut self) -> &mut MenuElement {
        &mut self.elements[self.index]
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.elements.len();
    }

    pub fn prev(&mut self) {
        self.index = (self.index + self.elements.len() - 1) % self.elements.len();
    }
}

#[derive(Debug, Clone)]
pub struct SettingsMenu {
    tabs: Vec<MenuTab>,
    index: usize,
}

impl SettingsMenu {
    pub fn new(_hw_config: HardwareConfig) -> Self {
        Self {
            tabs: vec![
                MenuTab {
                    name: "Internet".to_string(),
                    elements: vec![MenuElement::IpAddressEth, MenuElement::IpAddressWln],
                    index: 0,
                },
                MenuTab {
                    name: "Update".to_string(),
                    elements: vec![MenuElement::UpdateBtn("".to_string())],
                    index: 0,
                },
                MenuTab {
                    name: "Exit settings".to_string(),
                    elements: vec![MenuElement::ExitBtn],
                    index: 0,
                },
            ],
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

    pub fn get_item_mut(&mut self) -> &mut MenuTab {
        &mut self.tabs[self.index]
    }
}
