use reqwest::blocking::get;
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::from("");
    print!("Enter url: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut url)?;

    let url = url.trim();

    let mut file_name = String::from("");
    print!("Enter file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut file_name)?;

    let file_name = file_name.trim();

    let mut resp = get(url)?;
    // let mut text = resp.bytes()?;

    println!("Status: {}", resp.status());
    if resp.status().is_success() {
        let mut dwld = File::create(format!("{}.txt", file_name))?;

        io::copy(&mut resp, &mut dwld)?;
        println!("Download complete.");
    } else if resp.status().is_server_error() {
        println!("Server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }
    Ok(())
}
