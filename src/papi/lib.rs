use std::libc;

#[link_args = "-lpapi"]
extern {
    fn PAPI_is_initialized() -> libc::c_int;
}

#[fixed_stack_segment]
pub fn is_initialized() -> bool {
    let result = unsafe { PAPI_is_initialized() };
    result != 0
}
