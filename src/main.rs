extern crate papi;

fn main() {
    let counters = &[papi::Counter::PAPI_TOT_INS];
    let mut counters = unsafe {
        papi::CounterSet::new(counters)
    };

    let start = counters.read();
    let x = fib(14);
    let stop = counters.accum();

    println!("Computed fib(14) = {} in {} instructions.",
             x, stop[0] - start[0]);
}

fn fib(n: isize) -> isize {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}
