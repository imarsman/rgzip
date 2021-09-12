extern crate libflate;

// use std::env;
// use std::fs;
// use std::io;
// use std::path::Path;
use std::path::PathBuf;
use std::io;
// use std::io::{self, Write};
use std::io::{Read};
use libflate::gzip::{Encoder, Decoder};
use structopt::StructOpt;
// use libflate::gzip::Decoder;

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
    // let opt = Opt::from_args();
    // println!("{:?}", opt);
    // println!("input: {:?}", &opt.input);
    
    // let args: Vec<String> = env::args().collect();
    // if args.len() > 1 {
    //     let config = Config::new(&args);

    //     if Path::new(&config.filename).exists() {
    //         let contents = fs::read_to_string(&config.filename)
    //             .expect("Something went wrong reading the file");

    //         println!("filename: {}:\ncontents:\n{}", config.filename, contents);

    //         let mut input = io::stdin();
    //         let mut decoder = Decoder::new(&mut input).unwrap();
    //         io::copy(&mut decoder, &mut io::stdout()).unwrap();
    //     } else {
    //         println!("filename not found")
    //     }
    // } else {
    //     println!("no useful args")
    // }
 
    // let stdin = io::stdin();
    // // if stdin == 
    // if stdin. .lines().count() > 0 {
    
    //     for line in stdin.lock().lines() {
    //         println!("{}", line.unwrap());
    //     }
    // }

    // https://doc.servo.org/std/io/trait.BufRead.html
    // let stdin = io::stdin();
    // let mut stdin = stdin.lock();

    // Encoding
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    io::copy(&mut &b"Hello World!"[..], &mut encoder).unwrap();
    let encoded_data = encoder.finish().into_result().unwrap();

    // Decoding
    let mut decoder = Decoder::new(&encoded_data[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    assert_eq!(decoded_data, b"Hello World!");

    println!("{:?}", String::from_utf8(decoded_data).unwrap())

    // // Encoding

    // let mut vec = Vec::new();
    // let result = io::stdin().read_to_end(&mut vec);
    // if result.is_err() {
    //     // println!("error %{:?}", result);
    // }

    // let encoder = Encoder::new(vec).unwrap();
    // let encoded_data = encoder.finish().into_result().unwrap();

    // let result = io::stdout().write_all(&encoded_data);
    // if result.is_err() {
    //     println!("error %{:?}", result);
    // }

}