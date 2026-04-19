use reqwest::blocking::get;

fn main() -> Result<(), reqwest::Error>{
    let resp = get("https://www.rust-lang.org")?;

    if resp.status().is_success() {
        println!("Success!");
        println!("Status: {}", resp.status());
        // let body = resp.text()?;

        // println!("Response Body: {}", body);
        let c = match resp.content_length() {
            Some(data) => data,
            None => 0,
        };
        println!("Content length: {}", c);
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }
    Ok(())
}
