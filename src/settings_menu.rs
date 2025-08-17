use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
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
pub enum MenuElement {
    IpAddressWln,
    IpAddressEth,
    UpdateBtn,
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

                    // let ipv6 = interface.ips.iter().find(|item| item.is_ipv6());
                    // let ipv6 = if let Some(ipv6) = ipv6 {
                    //     ipv6.ip().to_string()
                    // } else {
                    //     "-".to_string()
                    // };

                    // MenuItem::Label(format!("Wi-Fi\nIPv4: {}\n IPv6: {}", ipv4, ipv6))
                    MenuItem::Label(format!("Wi-Fi\n{}", ipv4))
                } else {
                    MenuItem::Label(format!("No\ninterface\nfound"))
                }
            }
            Self::UpdateBtn => MenuItem::Button(format!("Update button")),
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

    pub fn get_elements(&self) -> Vec<MenuElement> {
        self.elements.clone()
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
    pub fn new() -> Self {
        Self {
            tabs: vec![
                MenuTab {
                    name: "Internet".to_string(),
                    elements: vec![MenuElement::IpAddressEth, MenuElement::IpAddressWln],
                    index: 0,
                },
                MenuTab {
                    name: "Update".to_string(),
                    elements: vec![MenuElement::UpdateBtn],
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
