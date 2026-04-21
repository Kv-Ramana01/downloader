use reqwest::{blocking::Client, header::RANGE};
use std::fs::remove_file;
use std::sync::{Arc, Mutex};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    thread,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::from("");
    print!("Enter url: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut url)?;

    let url = url.trim().to_string();

    let mut file_name = String::from("");
    print!("Enter file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut file_name)?;

    let file_name = file_name.trim().to_string();

    let mut td_input = String::new();
    print!("Enter thread count (1-16): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut td_input)?;

    let parsed: u64 = td_input.trim().parse().unwrap_or(4);
    let td: u64 = parsed.clamp(1, 16);

    println!("Choose extension:");
    println!("1. txt");
    println!("2. pdf");
    println!("3. jpg");
    println!("4. png");
    println!("5. zip");
    println!("6. custom");

    let mut ext_choice = String::new();
    print!("Enter choice: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut ext_choice)?;

    let ext = match ext_choice.trim() {
        "1" => "txt".to_string(),
        "2" => "pdf".to_string(),
        "3" => "jpg".to_string(),
        "4" => "png".to_string(),
        "5" => "zip".to_string(),
        "6" => {
            let mut custom = String::new();
            print!("Enter custom extension: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut custom)?;
            custom.trim().to_string()
        }
        _ => "txt".to_string(),
    };

    let final_name = format!("{}.{}", file_name, ext);

    let client = Client::new();
    let resp = client.get(&url).send()?;
    let mut handles = vec![];
    let completed = Arc::new(Mutex::new(0u64));
    println!("Status: {}", resp.status());
    let sz = match resp.content_length() {
        Some(data) => data,
        None => 0,
    };

    // let mut resp = get(url)?;
    // let mut text = resp.bytes()?;

    // println!("Status: {}", resp.status());
    if resp.status().is_success() {
        println!("Downloading...");
        let chunk = sz / td;
        for i in 0..td {
            let start = i * chunk;
            let end = if i == td - 1 {
                sz - 1
            } else {
                start + chunk - 1
            };
            let client_clone = client.clone();
            let url_owned = url.clone();

            let completed_clone = Arc::clone(&completed);
            let handle = thread::spawn(move || {
                // let response = client_clone
                //     .get(&url_owned)
                //     .header(RANGE, format!("bytes={}-{}", start, end))
                //     .send()
                //     .unwrap();
                // // println!("Thread {}: downloaded {} bytes.", i, end - start + 1);
                // let body = response.bytes().unwrap();
                // let mut file = File::create(format!("part{}.tmp", i)).unwrap();
                // file.write_all(&body).unwrap();
                let max_retries = 3;
                let mut success = false;

                for attempt in 1..=max_retries {
                    let result = client_clone
                        .get(&url_owned)
                        .header(RANGE, format!("bytes={}-{}", start, end))
                        .send();

                    match result {
                        Ok(response) => {
                            let body = response.bytes().unwrap();
                            let mut file = File::create(format!("part{}.tmp", i)).unwrap();
                            file.write_all(&body).unwrap();

                            success = true;
                            break;
                        }
                        Err(_) => {
                            println!("Chunk {} failed. Retry {}/{}", i, attempt, max_retries);
                        }
                    }
                }

                if !success {
                    println!("Chunk {} failed permanently.", i);
                    return false;
                }
                let mut done = completed_clone.lock().unwrap();
                *done += 1;

                let percent = (*done * 100) / td;
                print!("\rProgress: [{} / {}] {}%", *done, td, percent);
                io::stdout().flush().unwrap();
                // println!("Body length: {}", body.len());
                true
            });
            handles.push(handle);
            // println!("{} - {}", start, end);
        }
        let mut all_ok = true;

        for handle in handles {
            let result = handle.join().unwrap();

            if !result {
                all_ok = false;
            }
        }
        if !all_ok {
            println!("\nDownload failed. Merge aborted.");
            return Ok(());
        }
        File::create(&final_name).unwrap();
        let mut final_dest = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&final_name)
            .unwrap();
        for i in 0..td {
            let mut temp = File::open(format!("part{}.tmp", i)).unwrap();
            io::copy(&mut temp, &mut final_dest).unwrap();
            remove_file(format!("part{}.tmp", i))?;
        }

        // io::copy(&mut resp, &mut dwld)?;
        println!();
        println!("Download complete.");
    } else if resp.status().is_server_error() {
        println!("Server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }
    Ok(())
}

