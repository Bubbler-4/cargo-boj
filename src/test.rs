use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use console::Style;
use similar::{ChangeTag, TextDiff};

use crate::datastore::ProblemData;
use crate::optparse::BinOrCmd;
use crate::Result;

fn precompile_bin(bin: &str) {
    let mut command = Command::new("cargo");
    command.args(["build", "--bin", bin, "--release"]);
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn spawn_bin_or_cmd(bin_or_cmd: &BinOrCmd) -> Child {
    let mut command = match bin_or_cmd {
        BinOrCmd::Bin(bin) => {
            let mut path = "target/release".parse::<PathBuf>().unwrap();
            path.push(bin);
            path.set_extension(std::env::consts::EXE_EXTENSION);
            Command::new(&path)
        }
        BinOrCmd::Cmd(cmd) => {
            if cfg!(target_os = "windows") {
                let mut command = Command::new("cmd");
                command.arg("/C").arg(cmd);
                command
            } else {
                let mut command = Command::new("sh");
                command.arg("-c").arg(cmd);
                command
            }
        }
    };
    dbg!(&command);
    command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap()
}

fn run_test_case(bin_or_cmd: &BinOrCmd, spj: bool, input: &str, output: &str) -> Result<()> {
    let mut handle = spawn_bin_or_cmd(bin_or_cmd);
    let now = std::time::Instant::now();
    let stdin = handle.stdin.as_mut().unwrap();
    write!(stdin, "{}", input).unwrap();
    let stdout = handle.wait_with_output().unwrap();
    let elapsed = now.elapsed().as_micros();
    let result = String::from_utf8_lossy(&stdout.stdout).to_string();

    let output = output
        .trim_end()
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    let result = result
        .trim_end()
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    let diff = TextDiff::from_lines(&result, &output);
    let styles = if spj {
        (Style::new(), Style::new(), Style::new())
    } else {
        (Style::new().red(), Style::new().green(), Style::new())
    };
    let mut failed = false;
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => {
                    failed = true;
                    ("-", &styles.0)
                }
                ChangeTag::Insert => {
                    failed = true;
                    ("+", &styles.1)
                }
                ChangeTag::Equal => (" ", &styles.2),
            };
            print!("{}{}", style.apply_to(sign), style.apply_to(change));
        }
    }
    if !spj && failed {
        std::io::stdout().flush()?;
        eprintln!("{} on input:", Style::new().red().apply_to("Test failed"));
        eprintln!("{}", input);
        Err("")?
    }
    println!("Elapsed: {}.{:06}", elapsed / 1000000, elapsed % 1000000);
    Ok(())
}

pub fn test(problem_id: &str, bin_or_cmd: &BinOrCmd) -> Result<()> {
    if let BinOrCmd::Bin(bin) = bin_or_cmd {
        precompile_bin(bin);
    }
    let ProblemData { spj, testcases } = ProblemData::load(problem_id);
    for (input, output) in &testcases {
        run_test_case(bin_or_cmd, spj, input, output)?;
    }
    Ok(())
}
