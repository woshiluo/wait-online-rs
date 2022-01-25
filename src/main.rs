use nix::fcntl;
use nix::sys::stat;
use nix::unistd;

fn main() {
    let path = std::path::Path::new("/run/wait-online-rs/204.cond");

    match unistd::mkfifo::<std::path::Path>(&path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", path),
        Err(err) => println!("Error creating fifo: {}", err),
    }

    loop {
        let _ = sd_notify::notify(false, &[sd_notify::NotifyState::Status("Checking...")]).unwrap();
        match reqwest::blocking::get("https://g.cn/generate_204") {
            Ok(rep) if rep.status() == 204 => {
                break;
            }
            _ => (),
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    let _ = sd_notify::notify(false, &[sd_notify::NotifyState::Status("Connected")]).unwrap();
    let _ = sd_notify::notify(false, &[sd_notify::NotifyState::Ready]).unwrap();
    let _fd = fcntl::open::<std::path::Path>(
        &path,
        fcntl::OFlag::O_RDONLY,
        nix::sys::stat::Mode::empty(),
    )
    .unwrap();

    nix::unistd::pause();
}
