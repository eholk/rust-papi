extern mod papi;

fn fib(n: int) -> int {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}

#[test]
fn is_initialized() {
    papi::counters_in_use::cond.trap(|_| papi::Retry).inside(|| {
        papi::is_initialized();
    })
}

#[test]
fn num_counters() {
    papi::counters_in_use::cond.trap(|_| papi::Retry).inside(|| {
        papi::num_counters();
    })
}

#[test]
fn counter_set() {
    let mut counters = papi::CounterSet::new_wait([papi::PAPI_TOT_INS,
                                                   papi::PAPI_TOT_CYC]);
    
    let _start = counters.read();
    let _x = fib(14);
    let _stop = counters.accum();
}