use std::process;

fn main() {
    let config: beesolver::Config = argh::from_env();
    if let Err(err) = beesolver::run(config) {
        println!("{err}");
        process::exit(1);
    }
}
