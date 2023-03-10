use rand::seq::SliceRandom;
use std::vec;

fn main() {
    let keys = vec!["spec", "incr", "dpps", "fpde", "absp", "ufst", "satt", "seqr"];
    let av_keys = vec!["T*9[m/,$?VT7cZzdcPj", "DqhC~JKr/YDTDV8Ia'_", "tp,NiY62VAA+/*H.m9:", "85E|McdURMKw(zZe5TC"];
    println!("{:?}", keys.choose(&mut rand::thread_rng()));
    println!("{:?}", keys.choose(&mut rand::thread_rng()));
}
