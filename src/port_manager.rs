use serialport::TTYPort;

use std::{collections::HashMap, error::Error, path::PathBuf};

pub struct PortManager {
    ports: HashMap<PathBuf, TTYPort>,
}

impl PortManager {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
        }
    }

    pub fn get_port(&mut self, path: PathBuf, baud_rate: u32) -> Result<TTYPort, Box<dyn Error>> {
        if !self.ports.contains_key(&path) {
            let port: serialport::SerialPortBuilder =
                serialport::new(path.to_string_lossy(), baud_rate);

            let port: TTYPort = port.open_native()?;

            self.ports.insert(path.clone(), port);
        }

        let port = self.ports[&path]
            .try_clone_native()?;
        
        Ok(port)
    }
}
