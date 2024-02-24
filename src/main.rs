use error_chain::error_chain;
use regex::Regex;
use tokio::{time::{self, Duration}, io::{self, AsyncWriteExt}};

error_chain! {
    foreign_links {
        Io(io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let seconds = args.get(1).expect("Must be with seconds");
    let url = args.get(2).expect("Must be with url");

    assert_eq!(true, is_url_correct(url), "URL Parsing error");

    loop {
        check_health(url).await?;
        time::sleep(Duration::from_secs(seconds.parse().expect("Seconds must be a positive number"))).await;
    }
}

fn is_url_correct(url: &str) -> bool {
    let re = Regex::new(r"^(https|http)://(.+\.)+.+(/.+)*/*$").expect("nice regex :)");

    re.is_match(url)
}

async fn check_health(url: &str) -> Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("Checking {url}. ").as_bytes()).await?;
    stdout.flush().await?;

    let res = reqwest::get(url).await?;
    let status_code = res.status().as_str().to_string();

    print!("Result: ");

    match status_code.as_str() {
        "200" => println!("OK(200)"),
        _ => println!("Err({status_code})"),
    }

    Ok(())
}