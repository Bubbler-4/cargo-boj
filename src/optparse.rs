use bpaf::*;
use bpaf::batteries::cargo_helper;

use crate::datastore::Cookies;

pub enum Opts {
    Login(Login),
    Submit(Submit),
}

#[derive(Clone)]
pub struct Login {
    pub cookies: Option<Cookies>,
}

pub struct Submit {
    pub problem_id: String,
    pub language: Option<usize>,
}

pub fn cargo_boj_opts() -> Opts {
    let login = construct!(Opts::Login(cargo_boj_login()));
    let submit = construct!(Opts::Submit(cargo_boj_submit()));
    cargo_helper("boj", construct!([login, submit])).to_options().run()
}

fn cargo_boj_login() -> impl Parser<Login> {
    let bojautologin = long("bojautologin")
        .help("The value of cookie `bojautologin`")
        .argument("str");
    let onlinejudge = long("onlinejudge")
        .help("The value of cookie `OnlineJudge`")
        .argument("str");
    let cookies = construct!(Cookies { bojautologin, onlinejudge }).optional();
    construct!(Login { cookies })
        .to_options()
        .descr("Store BOJ login information for submitting solutions.")
        //.header("Footer")
        //.footer("Footer")
        .command("login")
}

fn cargo_boj_submit() -> impl Parser<Submit> {
    let problem_id = positional("PID").help("Problem ID");
    let language = short('l')
        .long("lang")
        .help("Language ID")
        .argument("LANG")
        .optional();
    construct!(Submit { problem_id, language })
        .to_options()
        .descr("Submit a solution to a BOJ problem.")
        //.footer("Footer 2")
        .command("submit")
}