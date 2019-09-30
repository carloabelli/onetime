use structopt::StructOpt;
use std::{
    error::Error,
    fs::File,
    io::{
        self,
        Read,
        Write,
    },
    path::PathBuf,
};

const CAPACITY: usize = 1024;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Encrypts/decrypts a file using a one-time pad.",
)]
struct Opt {
    #[structopt(
        parse(from_os_str),
    )]
    /// The input file to encrypt/decrypt
    input: PathBuf,

    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
    )]
    /// The output file (default: stdout)
    output: Option<PathBuf>,

    #[structopt(
        short = "p",
        long = "pad",
        parse(from_os_str),
    )]
    /// The pad to use for encryption/decryption
    pad: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut input = File::open(opt.input)?;
    let mut pad = File::open(opt.pad)?;

    if input.metadata()?.len() > pad.metadata()?.len() {
        panic!("file is larger than pad!");
    }

    let mut output: Box<dyn Write> = match opt.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    let mut input_buffer = vec![0u8; CAPACITY];
    let mut pad_buffer = vec![0u8; CAPACITY];
    let mut output_buffer = vec![0u8; CAPACITY];
    loop {
        let size = input.read(&mut input_buffer)?;
        if size == 0 {
            break;
        }
        pad.read_exact(&mut pad_buffer[..size])?;
        input_buffer.iter()
            .zip(pad_buffer.iter())
            .map(|(input_byte, pad_byte)| input_byte ^ pad_byte)
            .zip(output_buffer.iter_mut())
            .map(|(output_byte, output)| *output = output_byte);
        output.write_all(&output_buffer[..size])?;
    }

    Ok(())
}
