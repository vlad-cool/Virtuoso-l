use postcard;
use serde;
use std::{
    error::Error,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

const BUF_SIZE: usize = 512;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageTypeTX {
    Args(Vec<String>),
    Stdin(Vec<u8>),
    EOF,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageTypeRX {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
    EOF,
}

fn stream_write<T: Write, Msg: serde::Serialize>(
    msg: &Msg,
    stream: &mut T,
) -> Result<(), Box<dyn Error>> {
    let buf: Vec<u8> = postcard::to_stdvec(msg)?;
    let length_buf: [u8; 8] = buf.len().to_le_bytes();

    stream.write_all(&length_buf);
    stream.write_all(&buf);

    Ok(())
}

fn stream_receiver(mut stream: UnixStream) -> Result<(), Box<dyn Error>> {
    let mut stdout: std::io::Stdout = std::io::stdout();
    let mut stderr: std::io::Stderr = std::io::stderr();

    let mut length_buf: _ = (0 as usize).to_le_bytes();
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        stream.read_exact(&mut length_buf)?;
        let n: usize = usize::from_le_bytes(length_buf);
        stream.read_exact(&mut buf[0..n]);

        let msg: MessageTypeRX = postcard::from_bytes(&buf[0..n])?;

        match msg {
            MessageTypeRX::Stdout(msg) => {
                stdout.write_all(msg.as_slice())?;
            }
            MessageTypeRX::Stderr(msg) => {
                stderr.write_all(msg.as_slice())?;
            }
            MessageTypeRX::EOF => {
                break;
            }
        }
    }

    Ok(())
}

pub fn run_client(args: std::vec::Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut stream: UnixStream = UnixStream::connect("/tmp/bridge.sock")?;
    let mut stdin: std::io::Stdin = std::io::stdin();

    stream.set_nonblocking(true)?;

    let stream_clone: UnixStream = stream.try_clone()?;

    let thread_handler: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        stream_receiver(stream_clone);
    });

    stream_write(&MessageTypeTX::Args(args), &mut stream);

    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        let n: usize = stdin.read(&mut buf)?;

        let data: MessageTypeTX = if n == 0 {
            MessageTypeTX::EOF
        } else {
            MessageTypeTX::Stdin(buf.into())
        };

        stream_write(&data, &mut stream);

        if data == MessageTypeTX::EOF {
            break;
        }
    }

    thread_handler.join();

    Ok(())
}

pub fn run_server() -> Result<(), Box<dyn Error>> { 
    let mut stream: UnixStream = UnixStream::connect("/tmp/bridge.sock")?;

    

    Ok(())
}
