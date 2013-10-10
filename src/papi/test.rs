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
fn start_read_stop_counters() {
    let counters = [papi::PAPI_TOT_INS, papi::PAPI_TOT_CYC];
    papi::start_counters([papi::PAPI_TOT_INS, papi::PAPI_TOT_CYC]);
    let _x = fib(14);
    let values = [0, 0];
    papi::read_counters(values);
    papi::stop_counters(counters);
}

#[test]
fn counter_set() {
    let counters = papi::CounterSet::new([papi::PAPI_TOT_INS,
                                          papi::PAPI_TOT_CYC]);

    let _start = counters.read();
    let _x = fib(14);
    let _stop = counters.accum();
}