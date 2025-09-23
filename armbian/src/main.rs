use colored::Colorize;
use std::borrow::Cow;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

fn copy_file(src: &PathBuf, dst: &PathBuf) {
    eprintln!(
        "{} {} to {}",
        "copying".green(),
        src.display(),
        dst.display()
    );

    let output: std::process::Output = Command::new("cp")
        .arg(src)
        .arg(dst)
        .output()
        .expect(&"Failed to detach loop device".red());

    let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

    if !stdout.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stdout is not empty, stdout:".yellow(),
            stdout
        );
    }

    if !stderr.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stderr is not empty, stderr:".yellow(),
            stderr
        );
    }
}

fn get_file_size(file: &PathBuf) -> u64 {
    fs::metadata(&file)
        .expect("Failed to get file metadata")
        .len()
}

// Extends for at least size bytes, may be a bit more
fn extend_image(image: &PathBuf, size: u64) {
    eprintln!(
        "{} {} for at least {} bytes",
        "extending".green(),
        image.display(),
        size,
    );
    const CHUNK_SIZE: u64 = 4096;

    let file: std::fs::File = OpenOptions::new()
        .append(true)
        .open(image)
        .expect("Failed to open file");

    let mut writer: BufWriter<std::fs::File> = BufWriter::new(file);

    let buf: Vec<u8> = vec![0; CHUNK_SIZE as usize];
    let size: u64 = size / CHUNK_SIZE + 1;

    for _ in 0..size {
        writer.write(&buf).expect("Failed to write to file");
    }
    writer.flush().expect("Failed to flush file buffer");
}

fn add_fat32_partition(image: &PathBuf, offset: u64, size: u64) {
    eprintln!("{}", "Adding fat 32 partition".green());

    let output: std::process::Output = Command::new("parted")
        .arg("--script")
        .arg(image)
        .arg("mkpart")
        .arg("primary")
        .arg("fat32")
        .arg(format!("{}B", offset))
        .arg(format!("{}B", offset + size))
        .output()
        .expect(&"Failed to create fat32 partition".red());

    let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

    if !stdout.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stdout is not empty, stdout:".yellow(),
            stdout
        );
    }

    if !stderr.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stderr is not empty, stderr:".yellow(),
            stderr
        );
    }
}

struct Partition {}

struct VirtualDrive {
    device: PathBuf,
    partitions: Vec<Partition>,
}

impl VirtualDrive {
    pub fn new(image: &PathBuf) -> Self {
        eprintln!("{}", "Attaching image to loop device".green());

        let output: std::process::Output = Command::new("losetup")
            .arg("--partscan")
            .arg("--find")
            .arg("--show")
            .arg(image)
            .output()
            .expect(&"Failed to setup loop device".red());

        let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

        if !stderr.trim().is_empty() {
            eprintln!(
                "{} {}",
                "Warning: stderr is not empty, stderr:".yellow(),
                stderr
            );
        }

        if stdout.trim().is_empty() {
            eprintln!("{}", "Error: no loop device in stdout".red());
            panic!();
        }

        let device: PathBuf = PathBuf::from(stdout.trim());

        Self {
            device,
            partitions: vec![],
        }
    }
}

impl Drop for VirtualDrive {
    fn drop(&mut self) {
        eprintln!("{}", "Detaching image from loop device".green());

        self.partitions = vec![];

        let output: std::process::Output = Command::new("losetup")
            .arg("--detach")
            .arg(self.device.clone())
            .output()
            .expect(&"Failed to detach loop device".red());

        let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

        if !stdout.trim().is_empty() {
            eprintln!(
                "{} {}",
                "Warning: stdout is not empty, stdout:".yellow(),
                stdout
            );
        }

        if !stderr.trim().is_empty() {
            eprintln!(
                "{} {}",
                "Warning: stderr is not empty, stderr:".yellow(),
                stderr
            );
        }
    }
}

fn main() {
    const SECOND_PARTITION_SIZE: u64 = 2 * 1024 * 1024 * 1024;
    /*
    HELPERS

    Virtual partition struct
    Command runner
     */

    /*
    MAIN STEPS

    root password
    pi user
    setup pi user (groups)
    delete  /root/.not_logged_in_yet
    create /root/.no_rootfs_resize
    install packages (sway mingetty overlayroot libsdl2-2.0-0 libsdl2-gfx-1.0-0 libsdl2-image-2.0-0 libsdl2-mixer-2.0-0 libsdl2-net-2.0-0 libsdl2-ttf-2.0-0)
    configure mingetty
    udev gpio rules
    extend image with for partition
    create new partition
    configure sudo
    add Virtuoso
     */

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} source_image destination_image", args[0]);
        return;
    }

    let image: PathBuf = args[2].clone().into();

    copy_file(&args[1].clone().into(), &image);
    let original_size: u64 = get_file_size(&image);
    // extend_image(&image, 2 * 1024 * 1024 * 1024 + 1024);
    extend_image(&image, SECOND_PARTITION_SIZE);
    add_fat32_partition(&image, original_size, SECOND_PARTITION_SIZE);

    let drive: VirtualDrive = VirtualDrive::new(&image);

}
