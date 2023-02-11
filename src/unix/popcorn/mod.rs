pub type c_char = i8;
pub type wchar_t = i32;

pub type c_long = i64;
pub type c_ulong = u64;

pub type blkcnt_t = ::c_long;
pub type blksize_t = ::c_long;
pub type clock_t = ::c_long; //
pub type clockid_t = ::c_long;
pub type dev_t = ::c_ulong;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type ino_t = ::c_long;
pub type mode_t = ::c_int;
pub type nfds_t = ::c_ulong; //
pub type nlink_t = ::c_int;
pub type off_t = ::c_longlong; //
pub type pthread_t = *mut ::c_void;
pub type pthread_attr_t = *mut ::c_void;
pub type pthread_cond_t = *mut ::c_void;
pub type pthread_condattr_t = *mut ::c_void;
// Must be usize due to libstd/sys_common/thread_local.rs,
// should technically be *mut ::c_void
pub type pthread_key_t = usize;
pub type pthread_mutex_t = *mut ::c_void;
pub type pthread_mutexattr_t = *mut ::c_void;
pub type pthread_rwlock_t = *mut ::c_void;
pub type pthread_rwlockattr_t = *mut ::c_void;
pub type rlim_t = ::c_ulong;
pub type sa_family_t = ::c_uint;
pub type sem_t = *mut ::c_void;
pub type sigset_t = ::c_long;
pub type socklen_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type suseconds_t = i64;
pub type tcflag_t = ::c_uint;
pub type time_t = ::c_long;