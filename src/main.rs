extern crate libflate;

use std::process;
use atty::Stream;
use std::io::Cursor;
use std::path::PathBuf;
use std::io;use std::io::{Read};
use libflate::gzip::{Encoder, Decoder};
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
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,
}


fn main() {

    if atty::is(Stream::Stdin) {
        println!("no standard input - exiting");
        process::exit(1);
    }

    let mut vec = Vec::new();
    io::stdin().read_to_end(&mut vec).unwrap();

    // Encoding
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    let mut file = Cursor::new(vec);
    io::copy(&mut file, &mut encoder).unwrap();
    let encoded_data = encoder.finish().into_result().unwrap();

    // Decoding
    let mut decoder = Decoder::new(&encoded_data[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    // assert_eq!(decoded_data, b"Hello World!");

    println!("Got {:?}", String::from_utf8(decoded_data).unwrap())
}