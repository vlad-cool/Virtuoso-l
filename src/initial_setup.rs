use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, exit};

fn parse_config(path: PathBuf) -> std::io::Result<HashMap<String, String>> {
    let text: String = std::fs::read_to_string(path)?;
    let mut map: HashMap<String, String> = HashMap::new();

    for line in text.lines() {
        let line: &str = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(map)
}

fn write_config(path: PathBuf, config: &HashMap<String, String>) -> std::io::Result<()> {
    let mut out: String = String::new();
    for (key, value) in config {
        out.push_str(&format!("{}={}\n", key, value));
    }
    std::fs::write(path, out)
}

fn parse_argument(args: &Vec<String>, n: usize) -> usize {
    match args[n].as_str() {
        "--set-bootlogo" => {
            if n + 1 == args.len() {
                eprintln!("Usage: --set-bootlogo WIDTHxHEIGHT");
                return n + 1;
            }

            let splash_path: PathBuf = PathBuf::from(format!(
                "/home/pi/device_configurations/{}/splash",
                args[n + 1]
            ));

            if !splash_path.exists() {
                eprintln!(
                    "Splash directory '{}' does not exist",
                    splash_path.display()
                );
                return n + 2;
            }

            println!("Removing installed splash");
            let output: Output = Command::new("rm")
                .arg("-rf")
                .arg("/usr/share/plymouth/themes/Virtuoso")
                .output()
                .expect(&"Failed to remove splash");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            println!("Copying new splash");
            let output: Output = Command::new("cp")
                .arg("-r")
                .arg(splash_path)
                .arg("/usr/share/plymouth/themes/Virtuoso")
                .output()
                .expect(&"Failed to copy splash");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            let wallpaper_path: PathBuf = PathBuf::from(format!(
                "/home/pi/device_configurations/{}/wallpaper.png",
                args[n + 1]
            ));
            println!("Copying wallpaper");
            let output: Output = Command::new("cp")
                .arg(wallpaper_path)
                .arg("/home/pi")
                .output()
                .expect(&"Failed to copy wallpaper");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            n + 2
        }
        "--update-initramfs" => {
            println!("Generating initramfs");
            let output: Output = Command::new("update-initramfs")
                .arg("-u")
                .output()
                .expect(&"Failed to update initramfs");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            n + 1
        }
        "--protect-fs" => {
            let mut overlayroot_conf: File = OpenOptions::new()
                .append(true)
                .open("/etc/overlayroot.conf")
                .expect("Failed to open overlayroot.conf file");

            overlayroot_conf
                .write_all("overlayroot=\"tmpfs\"".as_bytes())
                .expect("Failed to write to overlayroot.conf file");

            n + 1
        }
        "--reboot" => {
            let output: Output = Command::new("reboot")
                .output()
                .expect(&"Failed to update initramfs");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            n + 1
        }
        "--enable-bootlogo" => {
            let config: Result<HashMap<String, String>, std::io::Error> =
                parse_config("/boot/armbianEnv.txt".into());

            match config {
                Ok(mut config) => {
                    config.insert("bootlogo".into(), "true".into());
                    if let Err(err) = write_config("/boot/armbianEnv.txt".into(), &config) {
                        eprintln!("Failed to write armbian env file, error: {err}")
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open armbian env file, error: {err}");
                }
            }

            let output: Output = Command::new("update-alternatives")
                .arg("--install")
                .arg("/usr/share/plymouth/themes/default.plymouth")
                .arg("default.plymouth")
                .arg("/usr/share/plymouth/themes/Virtuoso/Virtuoso.plymouth")
                .arg("200")
                .output()
                .expect(&"Failed to install theme");

            let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }
            if !stdout.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stdout is not empty, stderr:",
                    stdout.trim()
                );
            }

            let output: Output = Command::new("update-alternatives")
                .arg("--set")
                .arg("default.plymouth")
                .arg("/usr/share/plymouth/themes/Virtuoso/Virtuoso.plymouth")
                .output()
                .expect(&"Failed to set theme");

            let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

            if !stderr.trim().is_empty() {
                eprintln!(
                    "{} {}",
                    "Warning: stderr is not empty, stderr:",
                    stderr.trim()
                );
            }

            n + 1
        }
        "--config-overlays" => {
            if n + 1 == args.len() {
                eprintln!("Usage: --config-overlays OVERLAYS");
                return n + 1;
            }

            let config: Result<HashMap<String, String>, std::io::Error> =
                parse_config("/boot/armbianEnv.txt".into());

            match config {
                Ok(mut config) => {
                    config.insert("overlays".into(), args[n + 1].to_string());
                    if let Err(err) = write_config("/boot/armbianEnv.txt".into(), &config) {
                        eprintln!("Failed to write armbian env file, error: {err}")
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open armbian env file, error: {err}");
                }
            }

            n + 2
        }
        "--config-disp-mode" => {
            if n + 1 == args.len() {
                eprintln!("Usage: --config-disp-mode MODE");
                return n + 1;
            }

            let config: Result<HashMap<String, String>, std::io::Error> =
                parse_config("/boot/armbianEnv.txt".into());

            match config {
                Ok(mut config) => {
                    config.insert("disp_mode".into(), args[n + 1].to_string());
                    if let Err(err) = write_config("/boot/armbianEnv.txt".into(), &config) {
                        eprintln!("Failed to write armbian env file, error: {err}")
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open armbian env file, error: {err}");
                }
            }

            n + 2
        }
        "--config-extraargs" => {
            if n + 1 == args.len() {
                eprintln!("Usage: --config-extraargs ARGS");
                return n + 1;
            }

            let config: Result<HashMap<String, String>, std::io::Error> =
                parse_config("/boot/armbianEnv.txt".into());

            match config {
                Ok(mut config) => {
                    config.insert("extraargs".into(), args[n + 1].to_string());
                    if let Err(err) = write_config("/boot/armbianEnv.txt".into(), &config) {
                        eprintln!("Failed to write armbian env file, error: {err}")
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open armbian env file, error: {err}");
                }
            }

            n + 2
        }
        s => {
            eprintln!("Unknown argument {}", s);
            n + 1
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {}", args[0]);
        exit(1);
    }

    let mut n: usize = 1;

    while n < args.len() {
        n = parse_argument(&args, n);
    }
}
