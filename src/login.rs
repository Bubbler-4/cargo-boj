use tokio::runtime::Runtime;
use tokio::process::Command;
use tokio::time::Instant;
use std::process::Stdio;
use fantoccini::{ClientBuilder, Locator};
use serde_json::{Value::Object, json};
use crate::{Result, UA};
use crate::datastore::Cookies;

pub fn login_procedure(id: &str, password: &str) -> Result<Option<Cookies>> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        cargo_install_geckodriver().await?;
        let mut instance = geckodriver_instance().await?;
        let client = new_client().await?;
        let success = boj_login(&client, id, password).await?;
        let cookies = if success { Some(login_info(&client).await?) } else { None };
        client.close().await?;
        instance.kill().await?;
        Ok(cookies)
    })
}

async fn cargo_install_geckodriver() -> Result<()> {
    // cargo install geckodriver if not present
    print!("Installing geckodriver if not present... ");
    let start = Instant::now();
    let _status = Command::new("cargo")
        .args(["install", "geckodriver"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    println!("Done in {:.3}s", start.elapsed().as_secs_f64());
    Ok(())
}

async fn geckodriver_instance() -> Result<tokio::process::Child> {
    print!("Starting geckodriver... ");
    let start = Instant::now();
    let child = Command::new("geckodriver")
        .args(["--log", "trace"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    println!("Done in {:.3}s", start.elapsed().as_secs_f64());
    Ok(child)
}

async fn new_client() -> Result<fantoccini::Client> {
    print!("Starting new client from geckodriver... ");
    let start = Instant::now();
    let Object(cap) = json!({"moz:firefoxOptions": {"args": ["--headless"]}}) else { unreachable!() };
    let c = ClientBuilder::native().capabilities(cap).connect("http://localhost:4444").await?;
    println!("Done in {:.3}s", start.elapsed().as_secs_f64());
    Ok(c)
}

async fn boj_login(client: &fantoccini::Client, id: &str, password: &str) -> Result<bool> {
    client.set_ua(UA).await?;
    let login_page = "https://www.acmicpc.net/login?next=%2F";
    client.goto(login_page).await?;
    let url = client.current_url().await?;
    if url.as_str() != login_page {
        println!("Already logged in.");
        return Ok(true);
    }
    let el_id = client.find(Locator::Css(r#"[name="login_user_id"]"#)).await?;
    let el_pass = client.find(Locator::Css(r#"[name="login_password"]"#)).await?;
    let checkbox_autologin = client.find(Locator::Css(r#"[name="auto_login"]"#)).await?;
    let bt_login = client.find(Locator::Css(r#".btn-u.pull-right"#)).await?;
    el_id.send_keys(id).await?;
    el_pass.send_keys(password).await?;
    let autologin_checked = checkbox_autologin.prop("checked").await?;
    if autologin_checked != Some("true".to_string()) {
        checkbox_autologin.click().await?;
    }
    bt_login.click().await?;
    let url = client.current_url().await?;
    if url.as_str() != "https://www.acmicpc.net/" {
        println!("Login failed!");
        return Ok(false);
    }
    Ok(true)
}

async fn login_info(client: &fantoccini::Client) -> Result<Cookies> {
    let onlinejudge = client.get_named_cookie("OnlineJudge").await?.value().to_owned();
    // println!("OnlineJudge = {}", onlinejudge);
    let bojautologin = client.get_named_cookie("bojautologin").await?.value().to_owned();
    // println!("bojautologin = {}", bojautologin);
    Ok(Cookies { onlinejudge, bojautologin })
}