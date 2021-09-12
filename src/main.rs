extern crate libflate;

use atty::Stream;
use libflate::gzip::{Decoder, Encoder};
use std::fs;
use std::io;
use std::io::Cursor;
use std::io::{Read, Write};
use std::process;
use structopt::StructOpt;

// https://docs.rs/structopt/0.3.23/structopt/
#[derive(Debug, StructOpt)]
#[structopt(name = "rgzip", about = "gzip stdin or a file")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short = "D", long)]
    debug: bool,

    #[structopt(short = "d", long = "decompress", help = "decompress")]
    decompress: bool,

    #[structopt(short = "f", long = "force", help = "force overwrite")]
    force: bool,

    #[structopt(short = "k", long = "keep", help = "keep original file")]
    keep: bool,

    #[structopt(short = "i", default_value = "")]
    input: String,

    #[structopt(short = "c", help = "send to stdout instead of file")]
    stdout: bool,
}

fn read_file(path: &mut String) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    return contents;
}

// fn decompress() {}

// fn compress() {}

fn main() {
    let opt = Opt::from_args();
    println!("opt input {:?}", opt.input);

    let mut input_data = Vec::new();

    // handle input from stdin or file
    let mut input = opt.input;
    if input != "" {
        let contents: String = read_file(&mut input);
        input_data = contents.into_bytes();
    } else {
        if atty::is(Stream::Stdin) {
            println!("no standard input - exiting");
            process::exit(1);
        }
        io::stdin().read_to_end(&mut input_data).unwrap();
    }

    // decompress or compress
    if !opt.decompress {
        // Encoding
        let mut encoder = Encoder::new(input_data).expect("unable to read input data");
        let mut file = Cursor::new(Vec::new());
        io::copy(&mut file, &mut encoder).expect("unable to copy input data");
        let encoded_data = encoder
            .finish()
            .into_result()
            .expect("unable to encode input data");
        input_data = encoded_data
    } else {
        // Decoding
        let mut decoder = Decoder::new(&input_data[..]).expect("unable to read input data");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("unable to decode input data");
        input_data = decoded_data;
    }

    // make filenames (paths)
    let mut output_fn = input.clone();
    let original_fn = output_fn.clone();
    if !opt.decompress {
        output_fn = output_fn + ".gz";
    }

    // if output is supposed to be to stdout
    if opt.stdout == true {
        if atty::is(Stream::Stdout) {
            println!("no standard output - exiting");
            process::exit(1);
        }
        std::io::stdout()
            .write(&input_data)
            .expect("Unable to write to stdout");
        // if output to file
    } else {
        fs::write(output_fn, &input_data).expect("Unable to write file");
        // if not keeping the file delete
        if !opt.keep {
            fs::remove_file(original_fn).expect("Could not remove original file");
        }
    }
}
