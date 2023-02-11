pub const S_IFMT: ::c_int = 0x0F000;
pub const S_IFBLK: ::c_int = 0x06000;
pub const S_IFCHR: ::c_int = 0x02000;
pub const S_IFIFO: ::c_int = 0x01000;
pub const S_IFREG: ::c_int = 0x08000;
pub const S_IFDIR: ::c_int = 0x04000;
pub const S_IFLNK: ::c_int = 0x0A000;
pub const S_IFSOCK: ::c_int = 0x0C000;

pub const S_IRWXU: ::c_int = 0700;
pub const S_IRUSR: ::c_int = 0400;
pub const S_IWUSR: ::c_int = 0200;
pub const S_IXUSR: ::c_int = 0100;
pub const S_IRWXG: ::c_int = 070;
pub const S_IRGRP: ::c_int = 040;
pub const S_IWGRP: ::c_int = 020;
pub const S_IXGRP: ::c_int = 010;
pub const S_IRWXO: ::c_int = 07;
pub const S_IROTH: ::c_int = 04;
pub const S_IWOTH: ::c_int = 02;
pub const S_IXOTH: ::c_int = 01;
pub const S_ISUID: ::c_int = 04000;
pub const S_ISGID: ::c_int = 02000;
pub const S_ISVTX: ::c_int = 01000;

pub const S_IREAD: ::c_int = S_IRUSR;
pub const S_IWRITE: ::c_int = S_IWUSR;
pub const S_IEXEC: ::c_int = S_IXUSR;

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
