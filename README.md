# Cargo-BOJ

Test and submit solutions to BOJ problems

## Installation

For now:

```
git clone https://github.com/Bubbler-4/cargo-boj.git
cargo install --path=cargo-boj
```

## Usage

### Login

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

```
# Test main.rs against example test cases of problem 1000
$ cargo boj test 1000
# Test sol_1000.rs
$ cargo boj test 1000 --bin=sol_1000
# Test 1000.py
$ cargo boj test 1000 --cmd='python 1000.py'
```

### Submit

```
# Submit main.rs as Rust 2021 solution to problem 1000. Code open setting follows account preference
$ cargo boj submit 1000
# Submit sol_1000.rs as Rust 2018 solution, with code closed
$ cargo boj submit 1000 --path=src/bin/sol_1000.rs --lang=94 --code-open=n
```
