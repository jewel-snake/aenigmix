use std::io::{Write,Read};

mod kolakoski;

pub use kolakoski::Kolakoski as Encoder;

impl<T: Read+Write+std::io::Seek> Encoder<T> {
  pub fn filter<M:Read,O:Write>(&mut self, target: &mut M,output: &mut O) {
    let bytes = target.bytes();
    for byte in bytes {
      output.write(&[byte.unwrap() ^ self.get_byte()]).unwrap();
    }
  }
}
/*
pub fn chiper_file_with_cert<T:Read,U:Read>(target: &mut T, cert: &mut U) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("ciphered_output.bin").unwrap();
  filter(target, cert, &mut output);
}
pub fn decipher_file_with_cert<T:Read,U:Read>(target: &mut T, cert: &mut U) {
  let mut output = std::fs::OpenOptions::new().write(true).create(true).read(false).open("deciphered_output.txt").unwrap();
  filter(target, cert, &mut output);
}*/
pub fn filter<T:Read,U:Read,O:Write>(target: &mut T, cert: &mut U, output: &mut O) {
  let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
  let mut kolakoski = kolakoski::Kolakoski::new(cert,&mut file);
  let bytes = target.bytes();
  for byte in bytes {
    output.write(&[byte.unwrap() ^ kolakoski.get_byte()]).unwrap();
  }
}