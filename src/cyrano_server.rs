use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::match_info;
use crate::modules::{self, VirtuosoModule};
use crate::virtuoso_config::VirtuosoConfig;

enum Protocol {
    UNKNOWN,
    CYRANO1_0,
    CYRANO1_1,
}

impl Protocol {
    pub fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNWN"),
            Self::CYRANO1_0 => String::from("EFP1.0"),
            Self::CYRANO1_1 => String::from("EFP1.1"),
        }
    }
}

pub enum State {
    Fencing,
    Halt,
    Pause,
    Ending,
    Waiting,
}

struct FencerInfo {
    id: String,     //8
    name: String,   // 20
    nation: String, // 3
    score: u32,
    status: u8,
    yellow_card: u8,
    red_card: u8,
    light: u8,
    white_light: u8,
    medical_interventions: u8,
    reserve_introduction: u8,
    p_card: u8,
}

impl FencerInfo {
    pub fn new() -> Self {
        Self {
            id: String::from_str("").unwrap(),
            name: String::from_str("").unwrap(),   // 20
            nation: String::from_str("").unwrap(), // 3
            score: 0,
            status: 0,
            yellow_card: 0,
            red_card: 0,
            light: 0,
            white_light: 0,
            medical_interventions: 0,
            reserve_introduction: 0,
            p_card: 0,
        }
    }
    //     fn from_string(s: String) -> Result<Self, String> {
    //         let parts: Vec<&str> = s.split('|').collect();

    //         if parts.len() != 13 {
    //             Err("Wrong number of elements in input string with fencer info".to_string())
    //         } else {
    //             // let general_info: Vec<&str> = parts[0].split('|').collect();
    //             // todo!();
    //             // Err("Not implemented".to_string()) // TODO
    //             Ok(Self {
    //                 id: parts[1],
    //                 name: parts[2],   // 20
    //                 nation: parts[3], // 3
    //                 score: 0,
    //                 status: 0,
    //                 yellow_card: 0,
    //                 red_card: 0,
    //                 light: 0,
    //                 white_light: 0,
    //                 medical_interventions: 0,
    //                 reserve_introduction: 0,
    //                 p_card: 0,
    //             })
    //         }
    //     }

    pub fn to_1_0_string(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.id,
            self.name,
            self.nation,
            self.score,
            self.status,
            self.yellow_card,
            self.red_card,
            self.light,
            self.white_light,
            self.medical_interventions,
            self.reserve_introduction
        )
    }
    pub fn to_1_1_string(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.id,
            self.name,
            self.nation,
            self.score,
            self.status,
            self.yellow_card,
            self.red_card,
            self.light,
            self.white_light,
            self.medical_interventions,
            self.reserve_introduction,
            self.p_card
        )
    }
}

// struct RefereeInfo {
//     referee_id: u32,
//     referee_name: String,   // 20
//     referee_nation: String, // 3
// }

// struct ProtocolMessage {
//     protocol: Protocol,
//     command: String,

pub struct CyranoServer {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    match_info_modified_count: u32,
    config: Arc<Mutex<VirtuosoConfig>>,

    udp_socket: UdpSocket,

    protocol: Protocol,
    software_ip: Option<SocketAddr>,

    state: State,

    last_hello: Option<Instant>,
    online: bool,

    left_fencer: FencerInfo,
    right_fencer: FencerInfo,
}

impl VirtuosoModule for CyranoServer {
    fn run(&mut self) {
        self.udp_socket
            .set_nonblocking(true)
            .expect("Failed to set udp socket nonblocking");

        loop {
            let mut buf: [u8; 512] = [0u8; 512];
            match self.udp_socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => {
                    println!("Got {}", String::from_utf8(buf[0..size].to_vec()).unwrap());

                    let sss: String = String::from_utf8(buf.to_vec()).unwrap();

                    let parts: Vec<&str> = sss.split("|").collect();

                    println!("parts[2]: {}", parts[2]);

                    self.protocol = match parts[1] {
                        "EFP1" => Protocol::CYRANO1_0,
                        "EFP1.1" => Protocol::CYRANO1_1,
                        _ => Protocol::UNKNOWN,
                    };

                    self.software_ip = Some(src_addr);

                    match parts[2] {
                        "HELLO" => {
                            self.last_hello = Some(Instant::now());
                            self.send_full_info();
                        }
                        _ => {}
                    }
                }
                Err(_e) => {}
            }

            let data_updated: bool;

            {
                let match_info_data = self.match_info.lock().unwrap();
                data_updated = self.match_info_modified_count != match_info_data.modified_count;
                self.match_info_modified_count = match_info_data.modified_count;
            }

            if data_updated {
                self.send_full_info();
            }

            if let Some(last_hello) = self.last_hello {
                if Instant::now().duration_since(last_hello).as_secs() > 15 && self.online == true {
                    let mut match_info_data = self.match_info.lock().unwrap();
                    self.online = false;
                    match_info_data.cyrano_online = false;
                    match_info_data.modified_count += 1;
                } else if Instant::now().duration_since(last_hello).as_secs() <= 15
                    && self.online == false
                {
                    let mut match_info_data = self.match_info.lock().unwrap();
                    self.online = true;
                    match_info_data.cyrano_online = true;
                    match_info_data.modified_count += 1;
                }
            }

            thread::sleep(Duration::from_millis(50));
        }
    }

    fn get_module_type(&self) -> modules::Modules {
        modules::Modules::CyranoServer
    }
}

impl CyranoServer {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        config: Arc<Mutex<VirtuosoConfig>>,
    ) -> Self {
        let port: u16 = config.lock().unwrap().cyrano_server.cyrano_port;
        Self {
            match_info: match_info,
            match_info_modified_count: 0,

            config: config,

            state: State::Waiting,

            udp_socket: UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))
                .expect("couldn't bind udp socket to address"),

            protocol: Protocol::UNKNOWN,

            software_ip: None,

            last_hello: None,

            online: false,

            left_fencer: FencerInfo::new(),
            right_fencer: FencerInfo::new(),
        }
    }

    fn send_full_info(&mut self) {
        let match_info_data = self.match_info.lock().unwrap();

        self.left_fencer.score = match_info_data.left_score;
        self.right_fencer.score = match_info_data.right_score;

        let buf = format!(
            "|{}|INFO|7|aboba|2|7|8|2||{}||{}|{}|E||||%{}%{}",
            self.protocol.to_string(),
            match_info_data.timer,
            match_info_data.weapon,
            match_info_data.priority,
            match self.protocol {
                Protocol::UNKNOWN => String::from(""),
                Protocol::CYRANO1_0 => self.right_fencer.to_1_0_string(),
                Protocol::CYRANO1_1 => self.right_fencer.to_1_1_string(),
            },
            match self.protocol {
                Protocol::UNKNOWN => String::from(""),
                Protocol::CYRANO1_0 => self.left_fencer.to_1_0_string(),
                Protocol::CYRANO1_1 => self.left_fencer.to_1_1_string(),
            },
        );
        println!("{}", buf);
        if let Some(dest_ip) = self.software_ip {
            match self.udp_socket.send_to(buf.as_bytes(), dest_ip) {
                Ok(_) => {}
                Err(e) => println!("Failed to send UDP packet, error {}", e),
            }
        }
    }
}
