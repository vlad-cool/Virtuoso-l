use std::net::{SocketAddr, UdpSocket};
use std::str::{FromStr, Utf8Error};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// use log::{debug, error, info, trace, warn};

use crate::match_info;
use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;

enum Protocol {
    Unknown,
    Cyrano10,
    Cyrano11,
}

impl Protocol {
    pub fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNWN"),
            Self::Cyrano10 => String::from("EFP1.0"),
            Self::Cyrano11 => String::from("EFP1.1"),
        }
    }
}

enum CyranoError {
    BadRawMessage,
    BadMessageStruct,
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

struct MatchState {
    piste: String,
    competition_id: String,
    phase: u32,
    poul_tab: String,
    match_number: u32,
    round_number: u32,
}

pub struct CyranoServer {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    match_info_modified_count: u32,

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

                    // self.protocol = match parts[1] {
                    //     "EFP1" => Protocol::Cyrano10,
                    //     "EFP1.1" => Protocol::Cyrano11,
                    //     _ => Protocol::Unknown,
                    // };

                    self.software_ip = Some(src_addr);

                    match parts[2] {
                        "HELLO" => {
                            self.last_hello = Some(Instant::now());
                            let _ = self.parse_hello(&buf);
                            self.send_full_info();
                        }
                        _ => {}
                    }
                }
                Err(_e) => {}
            }

            let data_updated: bool;

            {
                let match_info_data: std::sync::MutexGuard<'_, match_info::MatchInfo> =
                    self.match_info.lock().unwrap();
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
}

impl CyranoServer {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        config: Arc<Mutex<VirtuosoConfig>>,
    ) -> Self {
        let port: u16 = config.lock().unwrap().cyrano_server.cyrano_port;
        Self {
            match_info,
            match_info_modified_count: 0,
            state: State::Waiting,
            udp_socket: UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))
                .expect("couldn't bind udp socket to address"),
            protocol: Protocol::Unknown,
            software_ip: None,
            last_hello: None,
            online: false,
            left_fencer: FencerInfo::new(),
            right_fencer: FencerInfo::new(),
        }
    }

    fn parse_hello(&mut self, buf: &[u8]) -> Result<(), CyranoError> {
        let msg: Result<&str, Utf8Error> = std::str::from_utf8(buf);
        let msg: String = match msg {
            Err(_) => {
                // error!("Error in CyranoServer::parse_hello function: bad buffer");
                return Err(CyranoError::BadRawMessage);
            }
            Ok(msg) => msg.to_string(),
        };

        let general_area: String = msg.split('%').next().unwrap().to_string();

        let general_area_parts: Vec<&str> = general_area.split("|").collect();

        self.protocol = match general_area_parts[1] {
            "EFP1" => Protocol::Cyrano10,
            "EFP1.1" => Protocol::Cyrano11,
            _ => Protocol::Unknown,
        };

        if general_area_parts[2] != "HELLO" {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: expected \"HELLO\", got \"{}\"", general_area_parts[1]);
            return Err(CyranoError::BadMessageStruct);
        }

        let piste: &str = general_area_parts[3];
        if piste.len() > 8 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of piste field is {} > 8", piste.len());
            return Err(CyranoError::BadMessageStruct);
        }

        let compe: &str = general_area_parts[4];
        if compe.len() > 8 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of compe field is {} > 8", compe.len());
            return Err(CyranoError::BadMessageStruct);
        }

        let phase: &str = general_area_parts[5];
        if phase.len() > 2 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of phase field is {} > 2", phase.len());
            return Err(CyranoError::BadMessageStruct);
        }
        let phase: u32 = match phase.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                // error!("Error in CyranoServer::parse_hello function: bad message struct: failed parsing phase number from {}, error: {}", phase, e);
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let poul_tab: &str = general_area_parts[6];
        if poul_tab.len() > 8 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of poul_tab field is {} > 8", poul_tab.len());
            return Err(CyranoError::BadMessageStruct);
        }

        let match_number: &str = general_area_parts[7];
        if match_number.len() > 3 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of match number field is {} > 3", match_number.len());
            return Err(CyranoError::BadMessageStruct);
        }
        let match_number: u32 = match match_number.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                // error!("Error in CyranoServer::parse_hello function: bad message struct: failed parsing match number from {}, error: {}", match_number, e);
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let round_number: &str = general_area_parts[7];
        if round_number.len() > 2 {
            // error!("Error in CyranoServer::parse_hello function: bad message struct: length of round number field is {} > 2", round_number.len());
            return Err(CyranoError::BadMessageStruct);
        }
        let round_number: u32 = match round_number.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                // error!("Error in CyranoServer::parse_hello function: bad message struct: failed parsing match number from {}, error: {}", round_number, e);
                return Err(CyranoError::BadMessageStruct);
            }
        };



        Ok(())
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
                Protocol::Unknown => String::from(""),
                Protocol::Cyrano10 => self.right_fencer.to_1_0_string(),
                Protocol::Cyrano11 => self.right_fencer.to_1_1_string(),
            },
            match self.protocol {
                Protocol::Unknown => String::from(""),
                Protocol::Cyrano10 => self.left_fencer.to_1_0_string(),
                Protocol::Cyrano11 => self.left_fencer.to_1_1_string(),
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
