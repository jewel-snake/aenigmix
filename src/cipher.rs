use std::fs::File;
use std::io::{Write,Read};
mod kolakoski;
//use kolakoski;

pub fn chiper_file_with_cert(target: &File, cert: &File) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("ciphered_output.bin").unwrap();
  gamma(target, cert, &mut output);
}
pub fn decipher_file_with_cert(target: &File, cert: &File) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("deciphered_output.txt").unwrap();
  gamma(target, cert, &mut output);
}
fn gamma(target: &File, cert: &File, output: &mut File) {
  let mut kolakoski = kolakoski::Kolakoski::from_file(&cert);
  let bytes = target.bytes();
  for byte in bytes {
    output.write(&[byte.unwrap() ^ kolakoski.get_byte()]).unwrap();
  }
}