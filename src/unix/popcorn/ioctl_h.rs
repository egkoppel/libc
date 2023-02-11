pub const FIONREAD: ::c_int = 0x541B;
pub const FIONBIO: ::c_int = 0x5421;
pub const FIONCLEX: ::c_int = 0x5450;
pub const FIOCLEX: ::c_int = 0x5451;

pub const SIOCGIFNAME: ::c_int = 0x8910;
pub const SIOCGIFCONF: ::c_int = 0x8912;
pub const SIOCGIFFLAGS: ::c_int = 0x8913;
pub const SIOCSIFFLAGS: ::c_int = 0x8914;
pub const SIOCGIFINDEX: ::c_int = 0x8933;

pub const SIOCPROTOPRIVATE: ::c_int = 0x89E0;
pub const SIOCDEVPRIVATE: ::c_int = 0x89F0;

extern "C" {
	#[no_mangle]
	pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
}
