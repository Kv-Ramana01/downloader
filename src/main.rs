use reqwest::blocking::get;
use std::io::{self, Write};
use std::fs::File;

fn main() -> Result<(), Box<dyn  std::error::Error>> {
    let mut url = String::from("");
    print!("Enter url: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut url)?;

    let url = url.trim();

    let mut file_name = String::from("");
    print!("Enter file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut file_name)?;

    let mut resp = get(url)?;
    // let mut text = resp.bytes()?; 
    
    let mut dwld = File::create(format!("{}.txt", file_name))?;

    io::copy(&mut resp, &mut dwld)?;
    println!("Download complete.");
    Ok(())
}
