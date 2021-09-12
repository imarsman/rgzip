extern crate libflate;

use atty::Stream;
use libflate::gzip::{Decoder, Encoder};
use std::env;
use std::fs;
use std::io;
use std::io::Cursor;
use std::io::{Read, Write};
// use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

// https://docs.rs/structopt/0.3.23/structopt/
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "gzip stdin or a file")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    // #[structopt(parse(from_os_str))]
    // input: Option<PathBuf>,

    // output: Option<PathBuf>,
    #[structopt(short)]
    input: String,

    /// Output file, stdout if not present
    // #[structopt(parse(from_os_str))]
    // #[structopt(short)]
    // output: Option<PathBuf>,
    #[structopt(short)]
    output: String,
    // /// Where to write the output: to `stdout` or `file`
    // #[structopt(short)]
    // out_type: String,
}

fn readFile(path: &mut String) -> String {
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
        let contents: String = readFile(&mut input);
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

    std::io::stdout().write(&encoded_data).unwrap();

    // println!("Got {:?}", String::from_utf8(decoded_data).unwrap())
}
