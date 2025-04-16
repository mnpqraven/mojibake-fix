use encoding_rs::SHIFT_JIS;
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("./sample/1.cue");
    let maybe_content = fs::read(file_path);
    if maybe_content.is_err() {
        println!("maybe_content err'd");
        return Ok(());
    }

    let bytes = maybe_content.unwrap();

    let (result, _encoding_used, had_errors) = SHIFT_JIS.decode(&bytes);
    println!("{}", result);
    assert!(!had_errors);
    Ok(())
}
