use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{
        self,
        Read,
        Write,
    },
};

const CAPACITY: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("onetime")
        .version("0.1")
        .author("Carlo Abelli <carlo@abelli.xyz>")
        .about("Encrypts/decrypts a file using a one-time pad.")
        .arg(Arg::with_name("FILE")
             .required(true)
             .help("The file to encrypt/decrypt"))
        .arg(Arg::with_name("PAD")
             .short("p")
             .long("pad")
             .takes_value(true)
             .value_name("PAD")
             .required(true)
             .help("The pad to use for encryption/decryption"))
        .arg(Arg::with_name("OUTPUT")
             .short("o")
             .long("output")
             .value_name("OUT")
             .help("The output file (default: stdout)"))
        .get_matches();

    let mut file = File::open(matches.value_of("FILE").unwrap())?;
    let mut pad = File::open(matches.value_of("PAD").unwrap())?;

    if file.metadata()?.len() > pad.metadata()?.len() {
        panic!("file is larger than pad!");
    }

    let mut out: Box<dyn Write> = match matches.value_of("OUTPUT") {
        Some(name) => Box::new(File::create(name)?),
        None => Box::new(io::stdout()),
    };

    let mut file_buffer = vec![0u8; CAPACITY];
    let mut pad_buffer = vec![0u8; CAPACITY];
    loop {
        let size = file.read(&mut file_buffer)?;
        if size == 0 {
            break;
        }
        pad.read_exact(&mut pad_buffer[..size])?;
        let out_buffer: Vec<u8> = pad_buffer.iter()
            .zip(file_buffer.iter())
            .map(|(pad_byte, file_byte)| pad_byte ^ file_byte)
            .collect();
        out.write_all(&out_buffer)?;
    }

    Ok(())
}
