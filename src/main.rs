mod login;
mod submit;
mod optparse;
mod datastore;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

// Planned features:
// cargo-boj login [--id=<id>]
//   login to BOJ with <id>. password is entered through a prompt.
//   if login is successful, id and cookies are stored for submitting solutions.
//   if id is supplied, it is used to log in to BOJ, and the id is stored for later runs.
//   if id is not supplied, the stored id is used; if it doesn't exist, the command fails.
// cargo-boj logout
//   delete id and cookies. may be useful if using cargo-boj on a shared computer.
// cargo-boj test <prob> [--bin=<bin>]
//   fetch sample tests for <prob> and run tests on the binary.
//   sample tests are cached by problem id.
//   if bin is supplied, uses its bin name. if set is also set, it is stored for later runs.
//   if bin is not supplied, the stored bin name is used; if it doesn't exist, defaults to `main`.
// cargo-boj submit <prob> [--path=<path>] [--lang-id=<lang>] [--code-open=(y|n|acc)]
//   submit the file at <path> as the solution to problem <prob>.
//   each option defaults to:
//   path = src/main.rs or src/bin/main.rs
//   lang-id = 113 (Rust 2021)
//   code-open = follow account default
// cargo-boj set [--bin=<bin>] [--path=<path>]
//   store settings for test binary name and submit file path.

use optparse::*;
use std::fs;

fn main() -> Result<()> {
    let opts = cargo_boj_opts();
    match opts {
        Opts::Login(Login { id }) => {
            let mut credentials = datastore::Credentials::load();
            let cur_id = if let Some(id) = id {
                // always remove existing credentials
                credentials.remove();
                credentials.update_id(&id);
                id
            } else {
                if let Some(id) = &credentials.id {
                    id.to_string()
                } else {
                    println!("ID argument missing and no previous ID found. Aborting.");
                    return Ok(());
                }
            };
            println!("BOJ ID: {}", cur_id);
            let password = rpassword::prompt_password("Enter BOJ Password: ")?;
            let cookie = login::login_procedure(&cur_id, &password)?;
            if let Some(cookie) = cookie {
                credentials.update_cookie(&cookie);
                println!("Login successful. You can use `cargo-boj submit` to submit solutions now.");
            }
        }
        Opts::Submit(Submit { problem_id, language }) => {
            let language = language.unwrap_or(113);
            let credentials = datastore::Credentials::load();
            let Some(cookies) = &credentials.cookies else {
                println!("Use `cargo-boj login` first to log in.");
                return Ok(());
            };
            let source = ["src/main.rs", "src/bin/main.rs"]
                .into_iter()
                .filter_map(|file| fs::read_to_string(file).ok())
                .next();
            let Some(source) = source else {
                println!("Neither src/main.rs nor src/bin/main.rs not found. Try running again at the crate root.");
                return Ok(());
            };
            submit::submit_solution(cookies, &problem_id, &source, language);
        }
    }
    Ok(())
}

// fn _main() -> Result<()> {
//     let should_login = false;
//     let id = "";
//     let password = "";
//     let (onlinejudge, bojautologin) = if should_login {
//         login::login_procedure(id, password)?
//     } else {
//         unimplemented!()
//     };
//     let problem_id = "2557";
//     let source = r#"fn main(){println!("Hello World!");}"#;
//     submit::submit_solution(&onlinejudge, &bojautologin, problem_id, source);
//     Ok(())
// }