use aenigmix::Encoder;
use std::path::PathBuf;
use std::fs::OpenOptions;

struct ArgsStruct {
  input: Option<PathBuf>,
  output: Option<PathBuf>,
  key: Option<PathBuf>
}

fn print_help() {
  println!("Usage:\ntodo");
}

fn parse_flag(args: &mut std::env::Args,strct: &mut ArgsStruct) {
  if let Some(a) = args.next() {
    println!("flag:{:?}",a);
    match a.as_str() {
      "-i" | "--input" => {
        if strct.input.is_some() {
          panic!("Same flag used twice!");
        }
        let input = std::path::Path::new(&args.next().expect("no path after input flag").as_str()).to_path_buf();
        strct.input = Some(input);
      },
      "-o" | "--output" => {
        if strct.output.is_some() {
          panic!("Same flag used twice!");
        }
        let output = std::path::Path::new(&args.next().expect("no path after output flag").as_str()).to_path_buf();
        strct.output = Some(output);
      },
      "-k" | "--key" => {
        if strct.key.is_some() {
          panic!("Same flag used twice!");
        }
        let key = std::path::Path::new(&args.next().expect("no path after key flag").as_str()).to_path_buf();
        strct.key = Some(key);
      },
      "-h" | "--help" => {
        if strct.input.is_none() && strct.output.is_none() && strct.key.is_none() {
          print_help();
        } else {
          panic!("Bad usage! Please refer help for more information!");
        }
      },
      _ => {
        panic!("Bad usage! Please refer help for more information!");
      }
    }
  }
}
fn main() {
  let mut args = std::env::args().into_iter();
  let mut args_struct = ArgsStruct{
    input: None,
    output: None,
    key: None,
  };
  args.next();
  for _ in 0..3 {
    parse_flag(&mut args, &mut args_struct);
  }
  if args.next().is_some() {
    panic!("Bad usage! Please refer help for more information!");
  }
  let mut input = OpenOptions::new().read(true).open(args_struct.input.unwrap()).unwrap();
  let mut output = OpenOptions::new().write(true).create(true).open(args_struct.output.unwrap()).unwrap();
  let mut key = OpenOptions::new().read(true).open(args_struct.key.unwrap()).unwrap();
  let inner = OpenOptions::new().read(true).write(true).create(true).open("kolakoski.bin").unwrap();
  let mut enc = Encoder::new(&mut key, inner);
  enc.filter(&mut input,&mut output);
  println!();
}