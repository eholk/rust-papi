extern mod papi;

fn main() {
    println!("Found {:d} counters.", papi::num_counters());
}