use std::env;

pub mod chapter;

fn main() {
   let args: Vec<String> = env::args().collect();

   let query = &args[1];
   println!("{:?}", query);
}