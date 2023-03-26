use std::process;

fn main() {
    if let Err(err) = beesolver::run() {
        println!("welp! {err}"); // todo better err
        process::exit(1);
    }
}
