use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 0 {
        loopstrings(args);
    } else {
        loopyes();
    }
}

fn loopyes() {
    loop {
        println!("yes");
    }
}

fn loopstrings(args: Vec<String>) {
    loopstring(args.connect(""));
}
fn loopstring(s: &str) {
    loop {
        println!("{}", s);
    }
}
