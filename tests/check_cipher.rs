use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use aenigmix::cipher;
#[test]
fn check_cipher() {
  initialize_files();
  cipher_file();
  let cert = File::open("cert.crt").unwrap();
  let ciphered = File::open("ciphered_output.bin").unwrap();
  cipher::decipher_file_with_cert(&ciphered,&cert);
  let deciphered = File::open("deciphered_output.txt").unwrap();
  let input = File::open("hello_world.txt").unwrap();
  for (a, b) in deciphered.bytes().zip(input.bytes()) {
    assert_eq!(a.unwrap(), b.unwrap());
  }
}

fn cipher_file() {
  let cert = File::open("cert.crt").unwrap();
  let file = File::open("hello_world.txt").unwrap();
  cipher::chiper_file_with_cert(&file,&cert);
}

fn initialize_files() {
  let mut cert = OpenOptions::new().write(true).create(true).open("cert.crt").unwrap();
  let mut file = OpenOptions::new().write(true).create(true).open("hello_world.txt").unwrap();
  file.write(b"hello world").unwrap();
  cert.write(b"AB").unwrap();
}