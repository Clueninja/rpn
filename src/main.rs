use std::env;
use rpn::rpn::rpn;

fn main() {
    let mut args:Vec<String> = env::args().collect();
    args.remove(0);
    // println!("{:?}", args);
    println!("{}", rpn(&args));
}
