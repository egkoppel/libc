mod dlfcn_h;
pub use self::dlfcn_h::*;
mod errno_h;
pub use self::errno_h::*;
mod fcntl_h;
pub use self::fcntl_h::*;
mod in_h;
pub use self::in_h::*;
mod ioctl_h;
pub use self::ioctl_h::*;
mod pthread_h;
pub use self::pthread_h::*;
mod seek_whence_h;
pub use self::seek_whence_h::*;
mod socket_h;
pub use self::socket_h::*;
mod stat_h;
pub use self::stat_h::*;
mod stdlib_h;
pub use self::stdlib_h::*;
mod time_h;
pub use self::time_h::*;
mod uio_h;
pub use self::uio_h::*;
mod un_h;
pub use self::un_h::*;
mod unistd_h;
pub use self::unistd_h::*;

pub type c_char = i8;
pub type wchar_t = i32;

pub type c_long = i64;
pub type c_ulong = u64;

pub type blkcnt_t = ::c_long;
pub type blksize_t = ::c_long;
pub type dev_t = ::c_ulong;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type ino_t = ::c_long;
pub type mode_t = ::c_int;
pub type nfds_t = ::size_t;
pub type nlink_t = ::c_int;
pub type off_t = ::c_long;
pub type rlim_t = ::c_ulong;
pub type sa_family_t = ::c_uint;
pub type sem_t = *mut ::c_void;
pub type sigset_t = ::c_long;
pub type socklen_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type suseconds_t = i64;
pub type tcflag_t = ::c_uint;

pub const NCCS: usize = 11;

#[repr(C)]
pub struct dirent {
    pub d_ino: ::ino_t,
    pub d_off: ::off_t,
    pub d_reclen: ::c_ushort,
    pub d_type: ::c_uchar,
    pub d_name: [::c_char; 1024],
}

#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::c_int,
    pub ai_family: ::c_int,
    pub ai_socktype: ::c_int,
    pub ai_protocol: ::c_int,
    pub ai_addrlen: ::socklen_t,
    pub ai_addr: *mut ::sockaddr,
    pub ai_canonname: *mut ::c_char,
    pub ai_next: *mut ::addrinfo,
}

#[repr(C)]
pub struct fd_set {
    fds_bits: [u8; 128],
}

#[repr(C)]
pub struct lconv {
    pub thousands_sep: *const ::c_char,
    pub grouping: *const ::c_char,
    pub decimal_point: *const ::c_char,
    pub mon_decimal_point: *const ::c_char,
    pub mon_thousands_sep: *const ::c_char,
    pub mon_grouping: *const ::c_char,
    pub positive_sign: *const ::c_char,
    pub negative_sign: *const ::c_char,
    pub currency_symbol: *const ::c_char,
    pub frac_digits: ::c_char,
    pub p_cs_precedes: ::c_char,
    pub n_cs_precedes: ::c_char,
    pub p_sep_by_space: ::c_char,
    pub n_sep_by_space: ::c_char,
    pub p_sign_posn: ::c_char,
    pub n_sign_posn: ::c_char,
    pub int_curr_symbol: *const ::c_char,
    pub int_frac_digits: ::c_char,
    pub int_p_cs_precedes: ::c_char,
    pub int_n_cs_precedes: ::c_char,
    pub int_p_sep_by_space: ::c_char,
    pub int_n_sep_by_space: ::c_char,
    pub int_p_sign_posn: ::c_char,
    pub int_n_sign_posn: ::c_char,
}

#[repr(C)]
pub struct statvfs {
    pub f_bsize: ::c_ulong,
    pub f_frsize: ::c_ulong,
    pub f_blocks: ::fsblkcnt_t,
    pub f_bfree: ::fsblkcnt_t,
    pub f_bavail: ::fsblkcnt_t,
    pub f_files: ::fsfilcnt_t,
    pub f_ffree: ::fsfilcnt_t,
    pub f_favail: ::fsfilcnt_t,
    pub f_fsid: ::c_ulong,
    pub f_flag: ::c_ulong,
    pub f_namemax: ::c_ulong,
}

#[repr(C)]
pub struct termios {
    pub c_iflag: ::tcflag_t,
    pub c_oflag: ::tcflag_t,
    pub c_cflag: ::tcflag_t,
    pub c_lflag: ::tcflag_t,
    pub c_cc: [::cc_t; ::NCCS],
    pub c_ispeed: ::speed_t,
    pub c_ospeed: ::speed_t,
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: ::sighandler_t,
    pub sa_mask: ::sigset_t,
    pub sa_flags: ::c_int,
    pub sa_restorer: ::Option<extern fn()>,
}
