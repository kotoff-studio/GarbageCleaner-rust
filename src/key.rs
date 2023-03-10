use rand::seq::SliceRandom;
use std::vec;

fn main() {
    let keys = vec!["spec", "incr", "dpps", "fpde", "absp", "ufst", "satt", "seqr"];
    println!("{:?}", keys.choose(&mut rand::thread_rng()));
}
