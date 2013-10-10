use libc::*;

#[link_args = "-lpapi"]
extern {
    fn PAPI_is_initialized() -> c_int;
}
