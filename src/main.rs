extern crate libflate;

use atty::Stream;
use libflate::gzip::{Decoder, Encoder};
use std::fs;
use std::io;
use std::io::Cursor;
use std::io::{Read, Write};
// use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

// https://docs.rs/structopt/0.3.23/structopt/
#[derive(Debug, StructOpt)]
#[structopt(name = "rgzip", about = "gzip stdin or a file")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short = "i", default_value = "")]
    input: String,

    #[structopt(short = "o", default_value = "")]
    output: String,
}

fn read_file(path: &mut String) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    return contents;
}

fn main() {
    let opt = Opt::from_args();
    println!("opt input {:?}", opt.input);
    println!("opt output {:?}", opt.output);

    let mut vec = Vec::new();

    let mut input = opt.input;
    if input != "" {
        let contents: String = read_file(&mut input);
        vec = contents.into_bytes();
    } else {
        if atty::is(Stream::Stdin) {
            println!("no standard input - exiting");
            process::exit(1);
        }
        io::stdin().read_to_end(&mut vec).unwrap();
    }

    // Encoding
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    let mut file = Cursor::new(vec);
    io::copy(&mut file, &mut encoder).unwrap();
    let encoded_data = encoder.finish().into_result().unwrap();

    // Decoding
    let mut decoder = Decoder::new(&encoded_data[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    let output = opt.output;
    let vec = Vec::new();
    if output != "" {
        fs::write(output, vec).expect("Unable to write file");
    } else {
        if atty::is(Stream::Stdout) {
            println!("no standard input - exiting");
            process::exit(1);
        }
        std::io::stdout().write(&encoded_data).unwrap();
    }
}
