//use std::fs::File;
use std::io::{Write,Read};
mod kolakoski;

pub fn chiper_file_with_cert<T:Read,U:Read>(target: &mut T, cert: &mut U) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("ciphered_output.bin").unwrap();
  gamma(target, cert, &mut output);
}
pub fn decipher_file_with_cert<T:Read,U:Read>(target: &mut T, cert: &mut U) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("deciphered_output.txt").unwrap();
  gamma(target, cert, &mut output);
}
fn gamma<T:Read,U:Read,O:Write>(target: &mut T, cert: &mut U, output: &mut O) {
  let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
  let mut kolakoski = kolakoski::Kolakoski::new(cert,&mut file);
  let bytes = target.bytes();
  for byte in bytes {
    output.write(&[byte.unwrap() ^ kolakoski.get_byte()]).unwrap();
  }
}