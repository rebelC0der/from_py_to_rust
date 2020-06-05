include!("intro.rs");
include!("strings.rs");
include!("dna_toolkit.rs");

fn main() {
    println!("Hello, I am Rust and I am fast");
    // _intro();
    // _strings();
    let dna = String::from(_gen_random_seq(20));
    println!("{}", dna);
    println!("{}", _transcription(&dna));
}
