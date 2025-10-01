use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::vec::Vec;

use crate::modules::HardwareConfig;

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
enum UpdateState {
    NoUpdate,
    Pending,
    Latest,
    Update,
    Downloading,
}

#[derive(Debug, Clone)]
pub enum MenuElement {
    IpAddressWln,
    IpAddressEth,
    UpdateBtn(UpdateState, String),
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
            Self::UpdateBtn(state, _) => MenuItem::Button(
                // match state {
                //     UpdateState::NoUpdate => "Pending",
                //     UpdateState::Pending => "Latest",
                //     UpdateState::Latest => "Update",
                //     UpdateState::Update => "Downloading",
                //     UpdateState::Downloading => "NoUpdate",
                // }
                "Update".into(),
            ),
        }
    }

    pub fn press(&mut self) {
        match self {
            Self::UpdateBtn(state, repo) => {
                *state = match state {
                    UpdateState::NoUpdate => {
                        // let client = Client::new();
                        // let resp = client
                        //     .get(repo.clone())
                        //     .header("User-Agent", "rust-reqwest")
                        //     .send()
                        //     // .await?
                        //     .error_for_status()?;
                        // fn get_latest_release_version(
                        //     owner: &str,
                        //     repo: &str,
                        // ) -> Result<String, reqwest::Error> {
                        //     let url = format!(
                        //         "https://api.github.com/repos/{}/{}/releases/latest",
                        //         owner, repo
                        //     );

                        //     let client = Client::new();
                        //     let resp = client
                        //         .get(&url)
                        //         .header("User-Agent", "rust-reqwest") // GitHub API requires a UA
                        //         .send()?
                        //         .error_for_status()?;

                        //     let release: Release = resp.json()?;
                        //     Ok(release.tag_name)
                        // // }

                        UpdateState::Pending
                    }
                    UpdateState::Pending => UpdateState::Latest,
                    UpdateState::Latest => UpdateState::Update,
                    UpdateState::Update => UpdateState::Downloading,
                    UpdateState::Downloading => UpdateState::NoUpdate,
                }
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
    pub fn new(hw_config: HardwareConfig) -> Self {
        Self {
            tabs: vec![
                MenuTab {
                    name: "Internet".to_string(),
                    elements: vec![MenuElement::IpAddressEth, MenuElement::IpAddressWln],
                    index: 0,
                },
                MenuTab {
                    name: "Update".to_string(),
                    elements: vec![MenuElement::UpdateBtn(
                        UpdateState::NoUpdate,
                        hw_config.update_repo.unwrap_or(
                            "https://api.github.com/vlad-cool/Virtuoso-l/releases/latest"
                                .to_string(),
                        ),
                    )],
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
