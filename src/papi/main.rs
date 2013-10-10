extern mod papi;

fn main() {
    let counters = papi::CounterSet::new([papi::PAPI_TOT_INS]);

    let start = counters.read();
    let x = fib(14);
    let stop = counters.accum();

    println!("Computed fib(14) = {:d} in {:d} instructions.",
             x, stop[0] - start[0]);
}

fn fib(n: int) -> int {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}
