pub const O_ACCMODE: ::c_int = 0x0007;
pub const O_EXEC: ::c_int = 1;
pub const O_RDONLY: ::c_int = 2;
pub const O_RDWR: ::c_int = 3;
pub const O_SEARCH: ::c_int = 4;
pub const O_WRONLY: ::c_int = 5;

pub const O_APPEND: ::c_int =    0x000008;
pub const O_CREAT: ::c_int =     0x000010;
pub const O_DIRECTORY: ::c_int = 0x000020;
pub const O_EXCL: ::c_int =      0x000040;
pub const O_NOCTTY: ::c_int =    0x000080;
pub const O_NOFOLLOW: ::c_int =  0x000100;
pub const O_TRUNC: ::c_int =     0x000200;
pub const O_NONBLOCK: ::c_int =  0x000400;
pub const O_DSYNC: ::c_int =     0x000800;
pub const O_RSYNC: ::c_int =     0x001000;
pub const O_SYNC: ::c_int =      0x002000;
pub const O_CLOEXEC: ::c_int =   0x004000;
pub const O_PATH: ::c_int =      0x008000;
pub const O_LARGEFILE: ::c_int = 0x010000;
pub const O_NOATIME: ::c_int =   0x020000;
pub const O_ASYNC: ::c_int =     0x040000;
pub const O_TMPFILE: ::c_int =   0x080000;
pub const O_DIRECT: ::c_int =    0x100000;

pub const F_DUPFD: ::c_int = 1;
pub const F_DUPFD_CLOEXEC: ::c_int = 2;
pub const F_GETFD: ::c_int = 3;
pub const F_SETFD: ::c_int = 4;
pub const F_GETFL: ::c_int = 5;
pub const F_SETFL: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_GETOWN: ::c_int = 10;
pub const F_SETOWN: ::c_int = 11;

pub const F_RDLCK: ::c_int = 1;
pub const F_UNLCK: ::c_int = 2;
pub const F_WRLCK: ::c_int = 3;

pub const FD_CLOEXEC: ::c_int = 1;

pub const F_SEAL_SHRINK: ::c_int = 0x0002;
pub const F_SEAL_GROW: ::c_int =   0x0004;
pub const F_SEAL_WRITE  : ::c_int =0x0008;
pub const F_SEAL_SEAL: ::c_int =   0x0010;
pub const F_ADD_SEALS: ::c_int =   1033;
pub const F_GET_SEALS: ::c_int =   1034;

pub const AT_EMPTY_PATH: ::c_int = 1;
pub const AT_SYMLINK_FOLLOW: ::c_int = 2;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 4;
pub const AT_REMOVEDIR: ::c_int = 8;
pub const AT_EACCESS: ::c_int = 512;

pub const AT_FDCWD: ::c_int = -100;