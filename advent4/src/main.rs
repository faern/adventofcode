extern crate crypto;

use std::sync::mpsc::channel;
use std::thread;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let key = "ckczppom";
    let (tx, rx) = channel();
    let threads = 4;
    for t in 0..threads {
        let t_tx = tx.clone();
        thread::spawn(move || {
            let offset = t;
            let step = threads;
            let mut digest = Md5::new();
            let mut buf = [0u8; 16];
            for i in 0.. {
                let num = offset + i * step;
                if mine(&mut digest, &mut buf, key, num) {
                    t_tx.send(num).unwrap();
                }
            }
        });
    }
    let result: u32 = rx.recv().unwrap();
    println!("Lowest integer giving an AdventCoin is {}", result);
}

fn mine(digest: &mut Md5, buf: &mut [u8], key: &str, i: u32) -> bool {
    digest.input(key.as_bytes());
    digest.input(format!("{}", i).as_bytes());
    digest.result(buf);
    digest.reset();
    buf[0] == 0 && buf[1] == 0 && buf[2] == 0
}
