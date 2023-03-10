use reqwest::{cookie::Jar, Url, blocking::Client};
use std::sync::Arc;
use std::io::Read;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use std::process::Stdio;
use fantoccini::{ClientBuilder, Locator};
use serde_json::{Value::Object, json};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    cargo_install_geckodriver().await?;
    let mut instance = geckodriver_instance().await?;
    boj_login().await?;
    instance.kill().await?;
    Ok(())
}

async fn cargo_install_geckodriver() -> Result<()> {
    // cargo install geckodriver if not present
    let _status = Command::new("cargo")
        .args(["install", "geckodriver"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Ok(())
}

async fn geckodriver_instance() -> Result<tokio::process::Child> {
    let child = Command::new("geckodriver")
        .args(["--log", "trace"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    sleep(Duration::from_secs(2)).await;
    Ok(child)
}

async fn boj_login() -> Result<()> {
    let Object(cap) = json!({"moz:firefoxOptions": {"args": ["--headless"]}}) else { unreachable!() };
    let c = ClientBuilder::native().capabilities(cap).connect("http://localhost:4444").await?;
    c.goto("https://www.acmicpc.net/").await?;
    let url = c.current_url().await?;
    println!("{}", url);
    c.close().await?;
    Ok(())
}

fn _failed_main() {
    let cookie = "bojautologin=7a54678d2c35bc7d8002c9d58dd6a4e5af86039f";
    let url = "https://acmicpc.net/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);
    let jar = Arc::new(jar);

    let client = Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
    .cookie_store(true)
    .cookie_provider(jar.clone())
    .build()
    .unwrap();
    let get = client.get("https://www.acmicpc.net/problem/2557");
    let mut res = get.send().unwrap();
    println!("{:?}", res);
    println!("{:?}", jar);
    let mut output = String::new();
    res.read_to_string(&mut output).unwrap();
    println!("{}", output);
}