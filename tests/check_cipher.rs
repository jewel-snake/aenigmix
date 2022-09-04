use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use aenigmix::Encoder;
#[test]
fn check_cipher() {
  initialize_files();
  cipher_file();
  let mut cert = File::open("cert.crt").unwrap();
  let mut ciphered = File::open("ciphered_output.bin").unwrap();
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("deciphered_output.txt").unwrap();
  let mut inner = std::fs::OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
  let mut kolakoski = Encoder::new(&mut cert,&mut inner);
  kolakoski.filter(&mut ciphered,&mut output);
  let deciphered = File::open("deciphered_output.txt").unwrap();
  let input = File::open("hello_world.txt").unwrap();
  for (a, b) in deciphered.bytes().zip(input.bytes()) {
    assert_eq!(a.unwrap(), b.unwrap());
  }
}

fn cipher_file() {
  let mut  cert = File::open("cert.crt").unwrap();
  let mut  file = File::open("hello_world.txt").unwrap();
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("ciphered_output.bin").unwrap();
  //filter(&mut file, &mut cert, &mut output);
  let mut inner = std::fs::OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
  let mut kolakoski = Encoder::new(&mut cert,&mut inner);
  kolakoski.filter(&mut file,&mut output);
}

fn initialize_files() {
  let mut cert = OpenOptions::new().write(true).create(true).open("cert.crt").unwrap();
  let mut file = OpenOptions::new().write(true).create(true).open("hello_world.txt").unwrap();
  file.write(b"hello world").unwrap();
  cert.write(b"AB").unwrap();
}