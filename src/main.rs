use clap::{Parser, command};
use encoding_rs::{EUC_JP, Encoding, GBK, ISO_2022_JP, SHIFT_JIS};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

/// Convert a text file from one (supposedly) japanese encoding to UTF-8
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// directory path
    #[arg(short, long)]
    dir: String,
    /// whether to write the result to a new file as "_filename"
    #[arg(short, long)]
    write: bool,
}

// this order matters
// GBK should be the final fallback, very rarely used and produced
// pixelated characters but still used sometimes nevertheless
const DECODERS: [&Encoding; 4] = [SHIFT_JIS, ISO_2022_JP, EUC_JP, GBK];

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dir = Path::new(&args.dir).read_dir().unwrap();
    for dir_entry in dir.into_iter() {
        // TODO: filter by extensions (text readable)
        let path = dir_entry.unwrap().path();

        for decoder in DECODERS.iter() {
            if let Ok(result) = try_decode(path.clone(), decoder) {
                let filename = path.file_name().unwrap().to_string_lossy();
                println!(
                    "DECODING FILE {} WITH {} SUCCESSFUL!",
                    filename,
                    decoder.name()
                );
                if args.write {
                    write_result(path, result);
                }
                break;
            }
        }
    }
    Ok(())
}

fn try_decode<P: AsRef<Path>>(
    path: P,
    encoding: &'static Encoding,
) -> Result<String, std::io::Error> {
    let maybe_content = fs::read(path);
    if let Err(_e) = maybe_content {
        panic!("FILE NOT FOUND");
    }

    let bytes = maybe_content.unwrap();

    let (result, _encoding_used, had_errors) = encoding.decode(&bytes);
    match had_errors {
        true => Err(std::io::Error::from(io::ErrorKind::InvalidData)),
        false => Ok(result.into_owned()),
    }
}

fn write_result(path: PathBuf, result: String) {
    let filename = path.file_name().unwrap().to_string_lossy();
    let mut next_name = String::from("_");
    next_name.push_str(&filename);
    let mut next_path = path.clone();
    next_path.set_file_name(next_name);
    let _ = fs::write(next_path, result);
}
