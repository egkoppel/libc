pub const CLOCK_REALTIME: ::c_int = 0;
pub const CLOCK_MONOTONIC: ::c_int = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::c_int = 2;
pub const CLOCK_THREAD_CPUTIME_ID: ::c_int = 3;
pub const CLOCK_MONOTONIC_RAW: ::c_int = 4;
pub const CLOCK_REALTIME_COARSE: ::c_int = 5;
pub const CLOCK_MONOTONIC_COARSE: ::c_int = 6;
pub const CLOCK_BOOTTIME: ::c_int = 7;

pub type clock_t = ::c_long;
pub type time_t = ::c_long;

#[repr(C)]
pub struct tm {
    pub tm_sec: ::c_int,
    pub tm_min: ::c_int,
    pub tm_hour: ::c_int,
    pub tm_mday: ::c_int,
    pub tm_mon: ::c_int,
    pub tm_year: ::c_int,
    pub tm_wday: ::c_int,
    pub tm_yday: ::c_int,
    pub tm_isdst: ::c_int,
    pub tm_gmtoff: ::c_long,
    pub tm_zone: *const ::c_char,
}

extern "C" {
	#[no_mangle]
	pub fn gettimeofday(result: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;

	#[no_mangle]
	pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
}
