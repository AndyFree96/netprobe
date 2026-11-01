use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://httpbin.org/get";
    print_usage(url);

    let response = reqwest::get(url).await?;
    let status = response.status();

    println!();
    println!("Status: {status}");
    Ok(())
}

fn print_usage(url: &str) {
    println!("netprobe v0.1.0");
    println!();
    println!("Target : {url}");
}
