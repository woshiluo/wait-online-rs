use nix::sys::stat;
use nix::{fcntl, unistd};

use sd_notify::NotifyState;

fn main() {
    let path = std::path::Path::new("/run/wait-online-rs/204.cond");

    match unistd::mkfifo::<std::path::Path>(path, stat::Mode::all()) {
        Ok(_) => (),
        Err(err) => match err {
            nix::errno::Errno::EEXIST => {
                use std::os::unix::fs::FileTypeExt;
                let metadata = path.metadata().unwrap();
                if !metadata.file_type().is_fifo() {
                    panic!("File Exists");
                }
            }
            _ => panic!("Err {}", err),
        },
    }

    loop {
        let _ = sd_notify::notify(false, &[NotifyState::Status("Checking...")]).unwrap();
        match reqwest::blocking::get("https://g.cn/generate_204") {
            Ok(rep) if rep.status() == 204 => break,
            Err(err) => println!("Failed connect: {}", err),
            _ => (),
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    let _ = sd_notify::notify(false, &[NotifyState::Status("Connected")]).unwrap();
    let _ = sd_notify::notify(false, &[NotifyState::Ready]).unwrap();
    let _fd =
        fcntl::open::<std::path::Path>(path, fcntl::OFlag::O_RDONLY, stat::Mode::empty()).unwrap();

    nix::unistd::pause();
}
