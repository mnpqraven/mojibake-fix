use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("./sample/1.cue");
    let maybe_content = fs::read(file_path);
    if let Ok(content) = maybe_content {
        let maybe = String::from_utf8(content.clone());
        println!("utf8 validation: {:?}", maybe);

        for (index, bytecode) in content.iter().enumerate() {
            let c = char::from(*bytecode);
            if (index >= 84) {
                print!("{}", c);
            }
        }
    }
}
