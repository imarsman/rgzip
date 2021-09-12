extern crate libflate;

use atty::Stream;
use glob::glob;
use libflate::gzip::{Decoder, Encoder};
use regex::Regex;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
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

    #[structopt(short = "f", long = "force", help = "force overwrite")]
    force: bool,

    #[structopt(short = "k", long = "keep", help = "keep original file")]
    keep: bool,

    #[structopt(short = "d", long = "decompress", help = "decompress")]
    decompress: bool,

    #[structopt(
        short = "i",
        default_value = "<filename>",
        help = "list of paths to files"
    )]
    input: Vec<String>,

    #[structopt(short = "c", help = "send to stdout instead of file")]
    stdout: bool,
}

fn read_file(path: &mut String) -> std::vec::Vec<u8> {
    let contents = fs::read(path).expect("Something went wrong reading the file");

    return contents;
}

fn write_file(path: &mut String, data: &mut std::vec::Vec<u8>) {
    fs::write(path, &data).expect("Unable to write file");
}

// decompress a byte vector - more easily tested
// https://docs.rs/libflate/0.1.9/libflate/gzip/struct.Decoder.html
fn decompress(input_data: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
    let mut decoder = Decoder::new(&input_data[..]).unwrap();
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();

    return buf;
}

// compress a byte vector - more easily tested
// https://docs.rs/libflate/0.1.25/libflate/gzip/struct.Encoder.html
fn compress(mut input_data: std::vec::Vec<u8>) -> std::vec::Vec<u8> {
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder.write_all(&input_data).unwrap();
    input_data = encoder.finish().into_result().unwrap();

    return input_data;
}

fn main() {
    let opt = Opt::from_args();

    let mut paths = Vec::new();

    for opt_path in opt.input.iter() {
        for entry in glob(opt_path).expect("Failed to read glob pattern") {
            let p = entry.unwrap();
            let p_str = p.into_os_string().into_string();
            if !paths.contains(&p_str) {
                paths.push(p_str);
            }
        }
    }

    let mut input_data = Vec::new();

    // if output is supposed to be to stdout handle that
    if opt.stdout == true {
        if atty::is(Stream::Stdout) {
            println!("no standard output - exiting");
            process::exit(1);
        }
        io::stdin().read_to_end(&mut input_data).unwrap();
        if opt.decompress {
            input_data = decompress(input_data);
        } else {
            input_data = compress(input_data);
        }
        std::io::stdout()
            .write(&input_data)
            .expect("Unable to write to stdout");
        process::exit(0);
    }

    // run through paths in input list
    for path in opt.input.iter() {
        // handle input from stdin or file
        // let mut input_fn = opt.input;
        let mut input_fn = path.clone();

        input_data = read_file(&mut input_fn);

        // decompress or compress
        if !opt.decompress {
            input_data = compress(input_data)
        } else {
            input_data = decompress(input_data)
        }

        // make filenames (paths)
        let mut output_fn = input_fn.clone();
        let original_fn = output_fn.clone();
        if !opt.decompress {
            output_fn = output_fn + ".gz";
        } else {
            if output_fn.contains(".gz") {
                let re = Regex::new("^(.*)\\.gz").expect("error with regular expression");
                let groups = re.captures(&output_fn).expect("problem extracting group");
                output_fn = String::from(groups.get(1).unwrap().as_str());
            }
        }

        // check for force
        if Path::new(&output_fn).exists() {
            if !opt.force {
                println!("skipping {:?}", output_fn);
                continue;
            }
        }

        // write the file
        write_file(&mut output_fn, &mut input_data);

        // if not keeping the file delete
        if !opt.keep {
            fs::remove_file(original_fn).expect("Could not remove original file");
        }
    }
}
