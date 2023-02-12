pub type pthread_t = *mut ::c_void;
pub type pthread_attr_t = *mut ::c_void;

#[repr(C)]
pub struct pthread_cond_t {
	__mlibc_seq: ::c_uint,
	__mlibc_flags: ::c_uint,
	__mlibc_clock: ::clockid_t,
}

#[repr(C)]
pub struct pthread_mutex_t {
	__mlibc_state: ::c_uint,
	__mlibc_recursion: ::c_uint,
	__mlibc_flags: ::c_uint,
	__mlibc_prioceiling: ::c_int,
}

#[repr(C)]
pub struct pthread_rwlock_t {
	__mlibc_m: ::c_uint,
	__mlibc_rc: ::c_uint,
	__mlibc_flags: ::c_uint,
}

pub type pthread_condattr_t = *mut ::c_void;
// Must be usize due to libstd/sys_common/thread_local.rs,
// should technically be *mut ::c_void
pub type pthread_key_t = usize;
pub type pthread_mutexattr_t = *mut ::c_void;
pub type pthread_rwlockattr_t = *mut ::c_void;

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;

pub const PTHREAD_MUTEX_INITIALIZER: ::pthread_mutex_t = pthread_mutex_t { __mlibc_state: 0, __mlibc_recursion: 0, __mlibc_flags: 0, __mlibc_prioceiling: 0 };
pub const PTHREAD_COND_INITIALIZER: ::pthread_cond_t = pthread_cond_t { __mlibc_seq: 0, __mlibc_flags: 0, __mlibc_clock: 0 };
pub const PTHREAD_RWLOCK_INITIALIZER: ::pthread_rwlock_t = pthread_rwlock_t { __mlibc_m: 0, __mlibc_rc: 0, __mlibc_flags: 0 };
pub const PTHREAD_STACK_MIN: ::size_t = 16384;

extern "C" {
	#[no_mangle]
	pub fn pthread_atfork(
		prepare: ::Option<unsafe extern "C" fn()>,
		parent: ::Option<unsafe extern "C" fn()>,
		child: ::Option<unsafe extern "C" fn()>,
	) -> ::c_int;

	#[no_mangle]
	pub fn pthread_create(
		thread: *mut ::pthread_t,
		attrp: *const ::pthread_attr_t,
		entry: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
		user_arg: *mut ::c_void,
	) -> ::c_int;

	#[no_mangle]
	pub fn pthread_condattr_setclock(
		attr: *mut pthread_condattr_t,
		clock_id: ::clockid_t,
	) -> ::c_int;

	#[no_mangle]
	pub fn pthread_setname_np(thread: ::pthread_t, name: *const ::c_char) -> ::c_int;
}
