# Cargo-BOJ

**IMPORTANT NOTE:** Cargo-BOJ is no longer maintained because it can no longer scrape BOJ pages. As of May 2024, BOJ uses AWS WAF challenge, which requires running Javascript to pass through. [Gaboja](https://github.com/Bubbler-4/rust-problem-solving/tree/main/gaboja) is a rewrite using Webdriver with similar functionality.

Test and submit solutions to BOJ (Baekjoon Online Judge) problems.

Defaults are geared towards Rust solutions, but non-Rust usage is supported as well.

## Prerequisites

A stable Rust toolchain.

## Installation

```
cargo install cargo-boj
```

You can use the same command to update to the latest version.

## Usage

The default usage of `test` and `submit` commands assume that `cargo boj` is being run at the crate root with
either `src/main.rs` or `src/bin/main.rs` being the solution file.
`src/main.rs` takes precedence if both are present.

### Login

Logging in to BOJ with ID and password cannot be automated because it is protected with reCaptcha.
So, the users are expected to log in on their own browser first, and then copy relevant cookies into `cargo-boj`.

```
$ cargo boj login
First log in to www.acmicpc.net on your browser with auto-login enabled.
Then copy and paste two cookies for www.acmicpc.net from your browser.
bojautologin: 3b1adXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
OnlineJudge: n00lXXXXXXXXXXXXXXXXXXXXXX
Cookies set.
```

or:

```
$ cargo boj login --bojautologin=3b1adXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX --onlinejudge=n00lXXXXXXXXXXXXXXXXXXXXXX
```

### Test

Tests your code against example test cases for the given problem.

* Test cases are fetched once and then cached. The cache can be refreshed with `-r, --refresh` flag.
* A colored diff is provided when a test fails with Wrong Answer.
* The exit status is 1 if and only if:
    * the problem is not one of "Special Judge (스페셜 저지)", "Score (점수)", "Two Steps (투 스텝)", or "Interactive (인터랙티브)", and
    * the output is not identical to the expected output and/or the program finished with a runtime error.
* `-p, --spj-prompt` flag is provided so that you can chain `test` and `submit` commands and still avoid submitting
    obviously incorrect solutions to SPJ problems.

```
# Test main.rs against example test cases of problem 1000
$ cargo boj test 1000

# Test src/bin/sol_1000.rs
$ cargo boj test 1000 --bin=sol_1000

# Test 1000.py
$ cargo boj test 1000 --cmd='python 1000.py'

# Test and submit problem 1008, but with user confirmation
$ cargo boj test 1008 --spj-prompt && cargo boj submit 1008
```

### Submit

Submits your code to BOJ using the credentials provided with `cargo boj login`.

The default language is `Rust 2021` (language ID 113). To submit solutions in other languages,
refer to [BOJ Help: language info](https://help.acmicpc.net/language/info).

```
# Submit main.rs as Rust 2021 solution to problem 1000. Code open setting follows account preference
$ cargo boj submit 1000

# Submit sol_1000.rs as Rust 2018 solution, with code closed
$ cargo boj submit 1000 --path=src/bin/sol_1000.rs --lang=94 --code-open=n
```

## Using within BOJ contest

When you open a problem in a contest, the address will be like `https://www.acmicpc.net/contest/problem/963/1`.
Then the problem ID for this problem is `963/1`.
You can use this ID in place of "problem ID" when using `cargo boj test` and `cargo boj submit`.

## Changelog

* 0.6.0
    * Change handling of "Two Step" and "Interactive" problems to not run the sample tests at all
    * Add `-r, --refresh` option to `cargo boj test`
* 0.5.1
    * Fix the issue around `-p` argument not following bpaf rules
* 0.5.0
    * Treat "Score" problems the same as Special Judge
    * Add `-p, --spj-prompt` option to `cargo boj test`
* 0.4.0
    * Fix some error messages from not showing when `cargo boj` exits
* 0.3.3
    * Fix `cargo boj test` trying to run incorrect executable name when the file to run is `src/main.rs`
* 0.3.1
    * Fix `cargo boj test` failing to find the built executable on Windows
* 0.3.0
    * Add support for BOJ contests