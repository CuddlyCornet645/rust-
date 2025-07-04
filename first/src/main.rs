extern crate stopwatch;
use stopwatch::{Stopwatch};
fn main() {
    let mut count = 0;
    let mut times: i32 = 1000000;
    let sw = Stopwatch::start_new();
    while times != 0 {
        print!("\r {} ", count);
        count += 1;
        times -= 1;
    }
    let time: f64 = sw.elapsed_ms() as f64 / 1000.0;

    println!("");
    println!("Dauerte {}s", time);
    println!("--- FERTIG ---");
}
