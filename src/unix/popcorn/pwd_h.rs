#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::c_char,
    pub pw_passwd: *mut ::c_char,
    pub pw_uid: ::uid_t,
    pub pw_gid: ::gid_t,
    pub pw_gecos: *mut ::c_char,
    pub pw_dir: *mut ::c_char,
    pub pw_shell: *mut ::c_char,
}

extern "C" {
    #[no_mangle]
    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
}