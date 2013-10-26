extern mod papi;

fn fib(n: int) -> int {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}

#[test]
fn is_initialized() {
    papi::is_initialized();
}

#[test]
fn num_counters() {
    papi::num_counters();
}

#[test]
fn counter_set() {
    let mut counters = papi::CounterSet::new([papi::PAPI_TOT_INS,
                                              papi::PAPI_TOT_CYC]);
    
    let _start = counters.read();
    let _x = fib(14);
    let _stop = counters.accum();
}