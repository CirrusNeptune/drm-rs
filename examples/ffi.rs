extern crate drm_ffi;

use drm_ffi as ffi;

use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};

#[derive(Debug)]
// This is our customized struct that implements the traits in drm.
struct Card(File);

// Need to implement AsRawFd before we can implement drm::Device
impl AsRawFd for Card {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl Card {
    fn open(path: &str) -> Self {
        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        Card(options.open(path).unwrap())
    }

    fn open_global() -> Self {
        Self::open("/dev/dri/card0")
    }
}

fn print_busid(fd: RawFd) {
    let mut buffer = Vec::new();
    let busid = ffi::get_bus_id(fd, Some(&mut buffer));
    println!("{:#?}", busid);
}

fn print_client(fd: RawFd) {
    let client = ffi::get_client(fd, 0);
    println!("{:#?}", client);
}

fn print_version(fd: RawFd) {
    let mut name = Vec::new();
    let mut date = Vec::new();
    let mut desc = Vec::new();

    let version = ffi::get_version(fd, Some(&mut name), Some(&mut date), Some(&mut desc));

    println!("{:#?}", version);
}

fn print_capabilities(fd: RawFd) {
    for cty in 1.. {
        let cap = ffi::get_capability(fd, cty);
        match cap {
            Ok(_) => println!("{:#?}", cap),
            Err(_) => break,
        }
    }
}

fn print_token(fd: RawFd) {
    let token = ffi::auth::get_magic_token(fd);
    println!("{:#?}", token);
}

/*
fn print_stats(fd: RawFd) {
    let stats = ffi::basic::get_stats(fd);
    println!("{:#?}", stats);
}
*/

fn main() {
    let card = Card::open_global();
    let fd = card.as_raw_fd();

    print_busid(fd);
    print_client(fd);
    print_version(fd);
    print_capabilities(fd);
    print_token(fd);
    //print_stats(fd);
}
