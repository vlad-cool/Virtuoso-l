use colored::Colorize;
use std::borrow::Cow;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn copy_file(src: &PathBuf, dst: &PathBuf) {
    eprintln!(
        "{} {} to {}",
        "Copying".green(),
        src.display(),
        dst.display()
    );

    let output: std::process::Output = Command::new("cp")
        .arg(src)
        .arg(dst)
        .output()
        .expect(&"Failed to copy file".red());

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

fn extend_image(image: &PathBuf, size: u64) {
    eprintln!(
        "{} {} for {} bytes",
        "Extending".green(),
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
    // let size: u64 = size / CHUNK_SIZE + 1;

    let mut remainder: u64 = size;

    while remainder >= CHUNK_SIZE {
        writer.write_all(&buf).expect("Failed to write to file");
        remainder -= CHUNK_SIZE;
    }

    writer
        .write_all(&vec![0u8; remainder as usize])
        .expect("Failed to write to file");

    // for _ in 0..size {
    // }
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
        .expect(&"Failed to create fat32 partition in partition table".red());

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

fn create_fat32_partition(partition: &PathBuf) {
    eprintln!("{}", "Creating fat 32 partition".green());

    let output: std::process::Output = Command::new("mkfs.vfat")
        .arg(partition)
        .output()
        .expect(&"Failed to create fat32 partition in partition table".red());

    let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

    if !stderr.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stderr is not empty, stderr:".yellow(),
            stderr
        );
    }
}

struct VirtualDrive {
    device: PathBuf,
    partitions: Vec<PathBuf>, // TODO make more rusty
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

    pub fn get_partition_path(&self, n: u8) -> Option<PathBuf> {
        let path: PathBuf = format!("{}p{}", self.device.display(), n).into();

        if path.exists() { Some(path) } else { None }
    }

    pub fn mount_partition(&mut self, n: u8) -> Option<PathBuf> {
        eprintln!("{} {}", "Mounting partition".green(), n);

        let mount_path: PathBuf = format!("mnt/{}", n).into();

        if mount_path.exists() {
            if let Err(err) = fs::remove_dir_all(&mount_path) {
                eprintln!("{}, err: {err}", &"Failed to delete mount directory".red());
                return None;
            }
        }
        if let Err(err) = fs::create_dir_all(&mount_path) {
            eprintln!("{}, err: {err}", &"Failed to create mount directory".red());
            return None;
        }

        let partition_path: PathBuf = if let Some(partition_path) = self.get_partition_path(n) {
            partition_path
        } else {
            return None;
        };

        let output: std::process::Output = Command::new("mount")
            .arg(partition_path)
            .arg(&mount_path)
            .output()
            .expect(format!("{} {}", "Failed to mount partition".red(), n).as_str());

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

        self.partitions.push(mount_path.clone());

        Some(mount_path)
    }
}

impl Drop for VirtualDrive {
    fn drop(&mut self) {
        eprintln!("{}", "Detaching image from loop device".green());

        for mount_path in &self.partitions {
            eprintln!("{} {}", "  Unmounting".green(), mount_path.display());

            let output: std::process::Output =
                Command::new("umount").arg(mount_path).output().expect(
                    format!(
                        "{} {}",
                        "Failed to umount partition".red(),
                        mount_path.display()
                    )
                    .as_str(),
                );

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

fn copy_qemu(path: &PathBuf, qemu_name: &str) -> PathBuf {
    eprintln!("{} {}", "Copying".green(), qemu_name);

    let output: std::process::Output = Command::new("which")
        .arg(qemu_name)
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
        eprintln!("{}", "Error: stdout is empty".red());
        panic!();
    }

    let qemu_path: PathBuf = PathBuf::from(stdout.trim());
    let mut bin_path: PathBuf = path.clone();
    bin_path.push("bin");
    copy_file(&qemu_path, &bin_path);

    bin_path.push(qemu_name);
    bin_path
}

fn remove_file(path: &PathBuf) {
    eprintln!("{} {}", "Removing".green(), path.display());

    let output: std::process::Output = Command::new("rm")
        .arg(path)
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

    if !stdout.trim().is_empty() {
        eprintln!(
            "{} {}",
            "Warning: stdout is not empty, stderr:".yellow(),
            stderr
        );
    }
}

fn run_in_target(target_command: &[&str], path: &PathBuf, qemu: &str, stdin: &str) {
    eprintln!("{} {}", "Running".green(), target_command.join(" "));

    let mut child: std::process::Child = Command::new("chroot")
        .arg(path)
        .arg(qemu)
        .args(target_command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&"Failed to spawn process".red());

    let child_stdin: &mut std::process::ChildStdin = child.stdin.as_mut().unwrap();
    child_stdin
        .write_all(stdin.as_bytes())
        .expect(&"Failed to wrie to stdin".red());

    let output: std::process::Output = child
        .wait_with_output()
        .expect(&"Failed to run command".red());

    let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

    if !stderr.trim().is_empty() {
        eprintln!("{}: {}", "stderr".yellow(), stderr);
    }

    if !stdout.trim().is_empty() {
        eprintln!("{}: {}", "stdout".green(), stdout);
    }

    if !output.status.success() {
        eprintln!(
            "{} {}",
            "Status: ".red(),
            output.status.code().unwrap_or(-1)
        )
    }
}

fn add_fat32_partition_to_fstab(device: &PathBuf, main_partition: &PathBuf) {
    eprintln!("{}", "Adding fat32 partition to fstab".green());

    let output: std::process::Output = Command::new("blkid")
        .arg("--match-tag")
        .arg("UUID")
        .arg("--output")
        .arg("value")
        .arg(device)
        .output()
        .expect(&"Failed to find uuid".red());

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
        eprintln!("{}", "Error: no uuid in stdout".red());
        panic!();
    }

    let uuid: &str = stdout.trim();

    let mut fstab_path: PathBuf = main_partition.clone();
    fstab_path.push("etc");
    fstab_path.push("fstab");

    let mut fstab: std::fs::File = OpenOptions::new()
        .append(true)
        .open(fstab_path)
        .expect("Failed to open file");

    fstab.write_all(format!("UUID={uuid}  /home/pi/Virtuoso  vfat  defaults,nofail,uid=1000,gid=1000,umask=022  0  2\n").as_bytes()).expect(&"Failed to write to file".red());
}

fn configure_dns(main_partition: &PathBuf) {
    eprintln!("{}", "Configuring dns".green());

    let mut conf_path: PathBuf = main_partition.clone();
    conf_path.push("etc");
    conf_path.push("resolv.conf");

    std::fs::remove_file(&conf_path).expect("Failed to remove conf file");

    let mut conf: std::fs::File = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&conf_path)
        .expect("Failed to open file");

    conf.write_all(
        "nameserver 8.8.8.8
"
        .as_bytes(),
    )
    .expect(&"Failed to write to file".red());
}

fn install_packages(main_partition: &PathBuf, qemu_name: &str) {
    eprintln!("{}", "Installing packages".green());

    run_in_target(&["/usr/bin/apt", "update"], &main_partition, qemu_name, "");
    run_in_target(
        &[
            "/usr/bin/apt",
            "install",
            "-y",
            "sway",
            "mingetty",
            "overlayroot",
            "libsdl2-2.0-0",
            "libsdl2-gfx-1.0-0",
            "libsdl2-image-2.0-0",
            "libsdl2-mixer-2.0-0",
            "libsdl2-net-2.0-0",
            "libsdl2-ttf-2.0-0",
        ],
        &main_partition,
        qemu_name,
        "",
    );
}

fn enable_autologin(main_partition: &PathBuf) {
    eprintln!("{}", "Enabling autologin".green());

    let mut conf_path: PathBuf = main_partition.clone();
    conf_path.push("etc");
    conf_path.push("systemd");
    conf_path.push("system");
    conf_path.push("getty@tty1.service.d");

    if !conf_path.exists() {
        fs::create_dir_all(&conf_path).expect(&"Failed to create config directory".red());
    }

    conf_path.push("override.conf");

    let mut conf: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .open(conf_path)
        .expect("Failed to open file");

    conf.write_all(
        "[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin pi --noclear %I $TERM
"
            .as_bytes(),
    )
    .expect(&"Failed to write to file".red());

    // run_in_target(
    //     &["/usr/bin/systemctl", "enable", "getty@tty1.service"],
    //     &main_partition,
    //     qemu,
    //     "",
    // );
}

fn configure_gpio(main_partition: &PathBuf, qemu: &str) {
    eprintln!("{}", "Configuring gpio".green());
    let mut conf_path: PathBuf = main_partition.clone();
    conf_path.push("etc");
    conf_path.push("udev");
    conf_path.push("rules.d");
    conf_path.push("97-gpio.rules");

    let mut conf: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .open(conf_path)
        .expect("Failed to open file");

    conf.write_all(
        "SUBSYSTEM==\"gpio\", KERNEL==\"gpiochip[0-4]\", GROUP=\"gpio\", MODE=\"0660\"
".as_bytes(),
    )
    .expect(&"Failed to write to file".red());

    run_in_target(
        &["/usr/sbin/groupadd", "--users", "pi", "gpio"],
        &main_partition,
        qemu,
        "",
    );
}

fn configure_sudo(main_partition: &PathBuf) {
    eprintln!("{}", "Configuring sudo".green());
    let mut conf_path: PathBuf = main_partition.clone();
    conf_path.push("etc");
    conf_path.push("sudoers");

    let mut conf: std::fs::File = OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .open(conf_path)
        .expect("Failed to open file");

    conf.write_all(
        "Defaults        env_reset
Defaults        mail_badpass
Defaults        secure_path=\"/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin\"
Defaults        !requiretty
Defaults        use_pty
root    ALL=(ALL:ALL) ALL
%admin  ALL=(ALL) ALL
%sudo   ALL=(ALL:ALL) ALL
@includedir /etc/sudoers.d
pi ALL=(ALL) NOPASSWD: /usr/bin/plymouth
pi ALL=(ALL) NOPASSWD: /home/pi/setup.sh
".as_bytes(),
    )
    .expect(&"Failed to write to file".red());
}

fn copy_assets(main_partition: &PathBuf, qemu: &str) {
    eprintln!("{}", "Copying assets".green());

    let mut target_path: PathBuf = main_partition.clone();
    target_path.push("home");
    target_path.push("pi");

    for entry in fs::read_dir("linux_assets").expect("Failed to open assets directory") {
        let entry: fs::DirEntry = entry.expect("Failed to get entry");
        let path: PathBuf = entry.path();

        let output: std::process::Output = Command::new("cp")
            .arg("-r")
            .arg(path)
            .arg(&target_path)
            .output()
            .expect(&"Failed to copy file".red());

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
    run_in_target(
        &["/usr/bin/chown", "--recursive", "pi:pi", "/home/pi"],
        &main_partition,
        qemu,
        "",
    );
    run_in_target(
        &["/usr/bin/chmod", "+x", "/home/pi/setup.sh"],
        &main_partition,
        qemu,
        "",
    );
}

fn copy_executables(second_partition: &PathBuf, main_executable: &PathBuf) {
    eprintln!("{}", "Copying executables".green());

    let mut path: PathBuf = second_partition.clone();
    path.push("app");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create directory")
    }

    copy_file(main_executable, &path);
}

fn main() {
    const SECOND_PARTITION_SIZE: u64 = 2 * 1024 * 1024 * 1024;
    const QEMU_NAME: &str = "qemu-arm-static";
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

    if args.len() < 4 {
        println!(
            "Usage: {} source_image destination_image main_executable",
            args[0]
        );
        return;
    }

    let image: PathBuf = args[2].clone().into();
    let main_executable: PathBuf = args[3].clone().into();

    copy_file(&args[1].clone().into(), &image);
    let original_size: u64 = get_file_size(&image);
    extend_image(&image, SECOND_PARTITION_SIZE + 512);
    add_fat32_partition(&image, original_size, SECOND_PARTITION_SIZE);

    let mut drive: VirtualDrive = VirtualDrive::new(&image);

    if let Some(partition_path) = drive.get_partition_path(2) {
        create_fat32_partition(&partition_path);
    } else {
        return;
    }

    let main_partition: PathBuf = if let Some(partition) = drive.mount_partition(1) {
        partition
    } else {
        return;
    };
    let second_partition: PathBuf = if let Some(partition) = drive.mount_partition(2) {
        partition
    } else {
        return;
    };

    add_fat32_partition_to_fstab(
        &drive.get_partition_path(2).expect(&"No partition".red()),
        &main_partition,
    );

    let qemu_path: PathBuf = copy_qemu(&main_partition, QEMU_NAME);

    // run_in_target(
    //     &["/usr/sbin/chpasswd"],
    //     &main_partition,
    //     QEMU_NAME,
    //     "root:VirtuosoRoot",
    // );

    // run_in_target(
    //     &[
    //         "/usr/sbin/adduser",
    //         "--quiet",
    //         "--disabled-password",
    //         "--gecos",
    //         "\"\"",
    //         "pi",
    //     ],
    //     &main_partition,
    //     QEMU_NAME,
    //     "",
    // );
    // run_in_target(
    //     &["/usr/sbin/chpasswd"],
    //     &main_partition,
    //     QEMU_NAME,
    //     "pi:Virtuoso",
    // );

    eprintln!(
        "{} sudo chroot mnt/1 /bin/bash /usr/lib/armbian/armbian-firstlogin",
        "Run and setup manually".cyan()
    );

    let mut input: String = String::new();
    eprintln!("{}", "Press return to continue".cyan());
    std::io::stdin().read_line(&mut input).unwrap();

    configure_dns(&main_partition);
    install_packages(&main_partition, QEMU_NAME);
    enable_autologin(&main_partition);
    configure_gpio(&main_partition, QEMU_NAME);
    configure_sudo(&main_partition);
    copy_assets(&main_partition, QEMU_NAME);
    copy_executables(&second_partition, &main_executable);

    let mut input: String = String::new();
    eprintln!("{}", "Press return to finish".cyan());
    std::io::stdin().read_line(&mut input).unwrap();

    remove_file(&qemu_path);
}
