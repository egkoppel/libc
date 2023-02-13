pub const S_IFMT: ::c_int = 0x0F000;
pub const S_IFBLK: ::c_int = 0x06000;
pub const S_IFCHR: ::c_int = 0x02000;
pub const S_IFIFO: ::c_int = 0x01000;
pub const S_IFREG: ::c_int = 0x08000;
pub const S_IFDIR: ::c_int = 0x04000;
pub const S_IFLNK: ::c_int = 0x0A000;
pub const S_IFSOCK: ::c_int = 0x0C000;

pub const S_IRWXU: ::c_int = 0o700;
pub const S_IRUSR: ::c_int = 0o400;
pub const S_IWUSR: ::c_int = 0o200;
pub const S_IXUSR: ::c_int = 0o100;
pub const S_IRWXG: ::c_int = 0o70;
pub const S_IRGRP: ::c_int = 0o40;
pub const S_IWGRP: ::c_int = 0o20;
pub const S_IXGRP: ::c_int = 0o10;
pub const S_IRWXO: ::c_int = 0o7;
pub const S_IROTH: ::c_int = 0o4;
pub const S_IWOTH: ::c_int = 0o2;
pub const S_IXOTH: ::c_int = 0o1;
pub const S_ISUID: ::c_int = 0o4000;
pub const S_ISGID: ::c_int = 0o2000;
pub const S_ISVTX: ::c_int = 0o1000;

pub const S_IREAD: ::c_int = S_IRUSR;
pub const S_IWRITE: ::c_int = S_IWUSR;
pub const S_IEXEC: ::c_int = S_IXUSR;

pub const UTIME_NOW: ::c_long = (((1 as ::c_long) << 30) - (1 as ::c_long));
pub const UTIME_OMIT: ::c_long = (((1 as ::c_long) << 30) - (2 as ::c_long));

#[repr(C)]
pub struct stat {
    pub st_dev: ::dev_t,
    pub st_ino: ::ino_t,
    pub st_mode: ::mode_t,
    pub st_nlink: ::nlink_t,
    pub st_uid: ::uid_t,
    pub st_gid: ::gid_t,
    pub st_rdev: ::dev_t,
    pub st_size: ::off_t,
    pub st_atime: ::time_t,
    pub st_atime_nsec: ::c_long,
    pub st_mtime: ::time_t,
    pub st_mtime_nsec: ::c_long,
    pub st_ctime: ::time_t,
    pub st_ctime_nsec: ::c_long,
    pub st_blksize: ::blksize_t,
    pub st_blocks: ::blkcnt_t,
}

impl core::clone::Clone for stat {

}

extern "C" {
	#[no_mangle]
	pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
}
