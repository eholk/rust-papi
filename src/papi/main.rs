extern mod papi;

fn fib(n: int) -> int {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    println!("Found {:d} counters.", papi::num_counters());

    println!("\nComputing some stuff...");
    papi::start_counters([papi::PAPI_TOT_INS, papi::PAPI_TOT_CYC]);
    let x = fib(14);
    let values = [0, 0];
    papi::read_counters(values);
    println!("fib(14) = {:d}", x);
    println!("Executed {:d} instructions in {:d} cycles ({:f} IPC)",
             values[0], values[1], values[0] as f64 / values[1] as f64);
}