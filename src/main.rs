use encoding_rs::{EUC_JP, Encoding, GBK, ISO_2022_JP, SHIFT_JIS};
use std::{error::Error, fs, io, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // this order matters
    // GBK should be the final fallback, very rarely used and produced
    // pixelated characters but still used sometimes nevertheless
    let decoders: [&'static Encoding; 4] = [SHIFT_JIS, ISO_2022_JP, EUC_JP, GBK];

    let dir = Path::new("./sample").read_dir().unwrap();
    for dir_entry in dir.into_iter() {
        let path = dir_entry.unwrap().path();

        for decoder in decoders.iter() {
            if let Ok(result) = try_decode(path.clone(), decoder) {
                println!(
                    "DECODING FILE {} WITH {} SUCCESSFUL!",
                    path.file_name().unwrap().to_string_lossy(),
                    decoder.name()
                );
                println!("{}", result);
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
