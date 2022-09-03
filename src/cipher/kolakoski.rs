use std::io::{Read,Write,Seek,SeekFrom};
use std::fs::{File, OpenOptions};
pub struct Kolakoski {
  file: File,
  file_read_ptr: u64,
  file_write_ptr: u64,
  seq_read_ptr: u64,
  byte_buf: u16,
  byte_buf_len: u8,
  last_written_is_zero: bool //one or zero
}

impl Kolakoski {
  //? sequence is red in bytes
  pub fn from_file(initial_sequence: &File) -> Kolakoski {
    let file = OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
    let mut kolakoski = Kolakoski{
      file: file,
      file_read_ptr: 0,
      file_write_ptr:0,
      seq_read_ptr:0,
      byte_buf_len: 0,
      byte_buf: 0,
      last_written_is_zero: true
    };
    let mut bytes = initial_sequence.bytes();
    let buf = bytes.next().unwrap().unwrap();
    if (buf >> 7 ) == 1 {
      kolakoski.last_written_is_zero = false;
    }
    kolakoski.produce_from_byte(buf);
    for byte in bytes {
      kolakoski.produce_from_byte(byte.unwrap());
    }
    kolakoski
  }
  fn gen_byte(&mut self) {
    self.file.seek(SeekFrom::Start(self.file_read_ptr)).unwrap();
    let mut inv_buf = [0_u8;1];
    self.file.read(&mut inv_buf).unwrap();
    self.file_read_ptr += 1;
    //let mut buf = inv_buf[0].reverse_bits();
    let mut buf = inv_buf[0];
    for _ in 0..8 {
      let last_bit = buf % 2;
      buf >>= 1;
      for _ in 0..=last_bit {
        self.byte_buf <<= 1;
        self.byte_buf_len += 1;
        if self.last_written_is_zero {
          self.byte_buf += 1;
        }
        #[cfg(debug_assertions)]
        print!("\n{:016b} {:2} {:08b} {}",self.byte_buf,self.byte_buf_len,buf,self.last_written_is_zero);
      }
      self.last_written_is_zero = !self.last_written_is_zero;
      if self.byte_buf_len > 8 {
        self.file.seek(SeekFrom::Start(self.file_write_ptr)).unwrap();
        let write_buf = [(((self.byte_buf >> (self.byte_buf_len - 8)) % (1 << 8)) as u8).reverse_bits()];
        #[cfg(debug_assertions)]
        print!(" {:08b}",write_buf[0]);
        self.file.write(&write_buf).unwrap();
        self.file_write_ptr += 1;
        self.byte_buf &= !(0b11111111 << (self.byte_buf_len - 8));
        self.byte_buf_len -= 8;
      }
    }
  }
  pub fn get_byte(&mut self) -> u8 {
    if self.seq_read_ptr >= self.file_write_ptr {
      self.gen_byte();
    }
    let mut buf = [0_u8; 1];
    self.file.seek(SeekFrom::Start(self.seq_read_ptr)).unwrap();
    self.file.read(&mut buf).unwrap();
    self.seq_read_ptr += 1;
    buf[0]
  }
  fn produce_from_byte(&mut self,mut byte: u8) {
    //byte = byte.reverse_bits();
    for _ in 0..8 {
      let last_bit = byte % 2;
      byte >>= 1;
      for _ in 0..=last_bit {
        self.byte_buf <<= 1;
        self.byte_buf_len += 1;
        if self.last_written_is_zero {
          self.byte_buf += 1;
        }
        #[cfg(debug_assertions)]
        print!("\n{:016b} {:2} {:08b} {}",self.byte_buf,self.byte_buf_len,byte,self.last_written_is_zero);
      }
      self.last_written_is_zero = !self.last_written_is_zero;
      if self.byte_buf_len > 8 {
        self.file.seek(SeekFrom::Start(self.file_write_ptr)).unwrap();
        let write_buf = [(((self.byte_buf >> (self.byte_buf_len - 8)) % (1 << 8)) as u8).reverse_bits()];
        #[cfg(debug_assertions)]
        print!(" {:08b}",write_buf[0]);
        self.file.write(&write_buf).unwrap();
        self.file_write_ptr += 1;
        self.byte_buf &= !(0b11111111 << (self.byte_buf_len - 8));
        self.byte_buf_len -= 8;
      }
    }
  }
  fn inject_byte(&mut self,byte: u8) {
    self.file.write(&[byte]).unwrap();
  }
  // ! must be a macro
  /*fn get_bytes(&mut self,number: usize) -> [u8] {
    let mut bytes = Vec::with_capacity(number);
    self.file.seek(SeekFrom::Start(self.seq_read_ptr)).unwrap();
    self.file.read_(&mut bytes).unwrap();
    self.seq_read_ptr += number as u64;
    bytes
  }*/
}

impl std::default::Default for Kolakoski {
  //? generate classic kolakoski sequence
  fn default() -> Self {
      let file = OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
      //file.write(&[0b11010010]).unwrap();
      let mut kolakoski =Kolakoski {
        file: file,
        file_read_ptr: 0,
        file_write_ptr: 0,
        seq_read_ptr: 0,
        byte_buf: 0,
        byte_buf_len: 0,
        last_written_is_zero: true
      };
      kolakoski.produce_from_byte(0b01001011);
      //kolakoski.byte_buf = 0;
      //kolakoski.byte_buf_len = 0;
      kolakoski
  }
}
#[cfg(test)]
mod tests {
  #[test]
  fn check_generator() {
    let mut kolakoski = super::Kolakoski::default();
    assert_eq!(kolakoski.get_byte(),0b10010011);
    assert_eq!(kolakoski.get_byte(),0b00110110);
    println!();
  }
} //22121121 22112112 12212211