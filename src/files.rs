use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn create(template: &str, path: &str) -> Result<(), Box<dyn Error>> {
    println!("Creating file {} with the template {:?}", path, template);
    let mut file = File::create(path)?;
    write!(file, "Teum!");

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
