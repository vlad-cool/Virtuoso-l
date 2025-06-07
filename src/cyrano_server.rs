use std::fmt::format;
use std::net::{SocketAddr, UdpSocket};
use std::str::{FromStr, Utf8Error};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use slint::BackendSelector;

use crate::match_info::{self, FencerInfo, ProgramState};
use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;
use crate::virtuoso_logger::Logger;

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

enum CyranoError<'a> {
    BadRawMessage,
    BadMessageStruct,
    BadFieldSize{limit: usize, actual: usize},
    BadIntField(&'a str),
}

pub enum State {
    Fencing,
    Halt,
    Pause,
    Ending,
    Waiting,
}

impl FencerInfo {
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
    logger: Logger,
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

            if self.match_info.lock().unwrap().program_state == ProgramState::Exiting {
                break;
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
        logger: Logger,
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
            logger,
        }
    }

    fn check_string_field(s: &str, max_len: usize) -> Result<&str, CyranoError> {
        if s.len() > max_len {
            Err(CyranoError::BadFieldSize{limit: max_len, actual: s.len()})
        } else {
            Ok(s)
        }
    }

    fn parse_value<T: FromStr>(s: &str, max_len: usize) -> Result<T, CyranoError> {
        match Self::check_string_field(s, max_len) {
            Ok(s) => match s.parse::<T>() {
                Ok(n) => Ok(n),
                Err(_e) => Err(CyranoError::BadIntField(s)),
            },
            Err(e) => Err(e),
        }
    }

    fn parse_hello(&mut self, buf: &[u8]) -> Result<(), CyranoError> {
        let function_name = "CyranoServer::parse_hello";
        let msg: Result<&str, Utf8Error> = std::str::from_utf8(buf);
        let msg: String = match msg {
            Err(_) => {
                self.logger
                    .error(format!("Error in {function_name} function: bad buffer"));
                return Err(CyranoError::BadRawMessage);
            }
            Ok(msg) => msg.to_string(),
        };

        let general_area: String = msg.split('%').next().unwrap().to_string();

        let general_area: Vec<&str> = general_area.split("|").collect();

        let protocol: Protocol = match general_area[1] {
            "EFP1" => Protocol::Cyrano10,
            "EFP1.1" => Protocol::Cyrano11,
            _ => {
                self.logger.error(format!(
                    "Error in {function_name}: got unknown protocol string: {}",
                    general_area[1]
                ));
                Protocol::Unknown
            }
        };

        if general_area[2] != "HELLO" {
            self.logger.error(format!("Error in {function_name} function: bad message struct: expected \"HELLO\", got \"{}\"", general_area[1]));
            return Err(CyranoError::BadMessageStruct);
        }

        let piste: &str = general_area[3];
        if piste.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of piste field is {} > 8", piste.len()));
            return Err(CyranoError::BadMessageStruct);
        }

        let competition_id: &str = general_area[4];
        if competition_id.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of compe field is {} > 8", competition_id.len()));
            return Err(CyranoError::BadMessageStruct);
        }

        // let phase: &str = general_area[5];
        // if phase.len() > 2 {
        //     self.logger.error(format!("Error in {function_name} function: bad message struct: length of phase field is {} > 2", phase.len()));
        //     return Err(CyranoError::BadMessageStruct);
        // }
        // let phase: u32 = match phase.parse::<u32>() {
        //     Ok(n) => n,
        //     Err(e) => {
        //         self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing phase number from {}, error: {}", phase, e));
        //         return Err(CyranoError::BadMessageStruct);
        //     }
        // };

        // let poul_tab: &str = general_area[6];
        // if poul_tab.len() > 8 {
        //     self.logger.error(format!("Error in {function_name} function: bad message struct: length of poul_tab field is {} > 8", poul_tab.len()));
        //     return Err(CyranoError::BadMessageStruct);
        // }

        // let match_number: &str = general_area[7];
        // if match_number.len() > 3 {
        //     self.logger.error(format!("Error in {function_name} function: bad message struct: length of match number field is {} > 3", match_number.len()));
        //     return Err(CyranoError::BadMessageStruct);
        // }
        // let match_number: u32 = match match_number.parse::<u32>() {
        //     Ok(n) => n,
        //     Err(e) => {
        //         self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing match number from {}, error: {}", match_number, e));
        //         return Err(CyranoError::BadMessageStruct);
        //     }
        // };

        // let round_number: &str = general_area[7];
        // if round_number.len() > 2 {
        //     self.logger.error(format!("Error in {function_name} function: bad message struct: length of round number field is {} > 2", round_number.len()));
        //     return Err(CyranoError::BadMessageStruct);
        // }
        // let round_number: u32 = match round_number.parse::<u32>() {
        //     Ok(n) => n,
        //     Err(e) => {
        //         self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing match number from {}, error: {}", round_number, e));
        //         return Err(CyranoError::BadMessageStruct);
        //     }
        // };

        self.protocol = protocol;

        let mut match_info = self.match_info.lock().unwrap();
        match_info.piste = piste.to_string();
        match_info.competition_id = competition_id.to_string();
        // match_info.phase = phase;
        // match_info.poul_tab = poul_tab.to_string();
        // match_info.match_number = match_number;
        // match_info.round_number = round_number;

        Ok(())
    }

    fn parse_disp(&mut self, buf: &[u8]) -> Result<(), CyranoError> {
        let function_name = "CyranoServer::parse_disp";
        let msg: Result<&str, Utf8Error> = std::str::from_utf8(buf);
        let msg: String = match msg {
            Err(_) => {
                self.logger
                    .error(format!("Error in {function_name} function: bad buffer"));
                return Err(CyranoError::BadRawMessage);
            }
            Ok(msg) => msg.to_string(),
        };

        let areas: Vec<&str> = msg.split('%').collect();

        if areas.len() != 3 {
            self.logger.error(format!(
                "Error in {function_name} function: number of areas is {} != 3",
                areas.len()
            ));
            return Err(CyranoError::BadMessageStruct);
        }

        let general_area: String = areas[0].to_string();
        let general_area: Vec<&str> = general_area.split("|").collect();

        let protocol: Protocol = match general_area[1] {
            "EFP1" => Protocol::Cyrano10,
            "EFP1.1" => Protocol::Cyrano11,
            _ => {
                self.logger.error(format!(
                    "Error in {function_name}: got unknown protocol string: {}",
                    general_area[1]
                ));
                Protocol::Unknown
            }
        };

        if general_area[2] != "HELLO" {
            self.logger.error(format!("Error in {function_name} function: bad message struct: expected \"HELLO\", got \"{}\"", general_area[1]));
            return Err(CyranoError::BadMessageStruct);
        }

        let piste: &str = general_area[3];
        if piste.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of piste field is {} > 8", piste.len()));
            return Err(CyranoError::BadMessageStruct);
        }

        let competition_id: &str = general_area[4];
        if competition_id.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of compe field is {} > 8", competition_id.len()));
            return Err(CyranoError::BadMessageStruct);
        }

        let phase: &str = general_area[5];
        if phase.len() > 2 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of phase field is {} > 2", phase.len()));
            return Err(CyranoError::BadMessageStruct);
        }
        let phase: u32 = match phase.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing phase number from {}, error: {}", phase, e));
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let poul_tab: &str = general_area[6];
        if poul_tab.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of poul_tab field is {} > 8", poul_tab.len()));
            return Err(CyranoError::BadMessageStruct);
        }

        let match_number: &str = general_area[7];
        if match_number.len() > 3 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of match number field is {} > 3", match_number.len()));
            return Err(CyranoError::BadMessageStruct);
        }
        let match_number: u32 = match match_number.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing match number from {}, error: {}", match_number, e));
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let round_number: &str = general_area[7];
        if round_number.len() > 2 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of round number field is {} > 2", round_number.len()));
            return Err(CyranoError::BadMessageStruct);
        }
        let round_number: u32 = match round_number.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing match number from {}, error: {}", round_number, e));
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let right_fencer_area: String = areas[1].to_string();
        let right_fencer_area_parts: Vec<&str> = right_fencer_area.split("|").collect();
        let right_fencer_id: &str = right_fencer_area_parts[1];
        if right_fencer_id.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of right fencer id is {} > 8", right_fencer_id.len()));
            return Err(CyranoError::BadMessageStruct);
        }
        let right_fencer_id: u32 = match right_fencer_id.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing right fencer id from {}, error: {}", round_number, e));
                return Err(CyranoError::BadMessageStruct);
            }
        };

        let left_fencer_area: String = areas[1].to_string();
        let left_fencer_area_parts: Vec<&str> = left_fencer_area.split("|").collect();
        let left_fencer_id: &str = left_fencer_area_parts[1];
        if left_fencer_id.len() > 8 {
            self.logger.error(format!("Error in {function_name} function: bad message struct: length of left fencer id is {} > 8", left_fencer_id.len()));
            return Err(CyranoError::BadMessageStruct);
        }
        let left_fencer_id: u32 = match left_fencer_id.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                self.logger.error(format!("Error in {function_name} function: bad message struct: failed parsing left fencer id from {}, error: {}", round_number, e));
                return Err(CyranoError::BadMessageStruct);
            }
        };

        self.protocol = protocol;

        let mut match_info = self.match_info.lock().unwrap();
        match_info.piste = piste.to_string();
        match_info.competition_id = competition_id.to_string();
        match_info.phase = phase;
        match_info.poul_tab = poul_tab.to_string();
        match_info.match_number = match_number;
        match_info.round_number = round_number;
        match_info.right_fencer.id = right_fencer_id;
        match_info.left_fencer.id = left_fencer_id;

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
