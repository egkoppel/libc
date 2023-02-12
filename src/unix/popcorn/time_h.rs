pub const CLOCK_REALTIME: ::clockid_t = 0;
pub const CLOCK_MONOTONIC: ::clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: ::clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: ::clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: ::clockid_t = 6;
pub const CLOCK_BOOTTIME: ::clockid_t = 7;

pub type clock_t = ::c_long;
pub type clockid_t = ::c_long;
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
