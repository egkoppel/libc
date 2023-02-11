pub const RTLD_LOCAL: ::c_int = 0;
pub const RTLD_NOW: ::c_int = 1;
pub const RTLD_GLOBAL: ::c_int = 2;
pub const RTLD_NOLOAD: ::c_int = 4;
pub const RTLD_NODELETE: ::c_int = 8;
pub const RTLD_DEEPBIND: ::c_int = 16;
pub const RTLD_LAZY: ::c_int = 32;

#[repr(C)]
pub struct Dl_info {
    pub dli_fname: *const ::c_char,
    pub dli_fbase: *mut ::c_void,
    pub dli_sname: *const ::c_char,
    pub dli_saddr: *mut ::c_void,
}
