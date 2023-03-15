use bpaf::*;
use bpaf::batteries::cargo_helper;

pub enum Opts {
    Login(Login),
    Submit(Submit),
}

pub struct Login {
    pub id: Option<String>,
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
    let id = short('i')
        .long("id")
        .help("BOJ ID")
        .argument("ID")
        .optional();
    construct!(Login { id })
        .to_options()
        .descr("Log in to BOJ to submit solutions.")
        .header("Footer")
        .footer("Footer")
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
        .footer("Footer 2")
        .command("submit")
}