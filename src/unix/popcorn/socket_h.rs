pub const SCM_CREDENTIALS: ::c_int = 0x02;

pub const SOCK_DGRAM: ::c_int = 1;
pub const SOCK_RAW: ::c_int = 2;
pub const SOCK_SEQPACKET: ::c_int = 3;
pub const SOCK_STREAM: ::c_int = 4;
pub const SOCK_NONBLOCK: ::c_int = 0x10000;
pub const SOCK_CLOEXEC: ::c_int = 0x20000;
pub const SOCK_RDM: ::c_int = 0x40000;

pub const SOL_SOCKET: ::c_int = 1;

pub const SO_ACCEPTCONN: ::c_int = 1;
pub const SO_BROADCAST: ::c_int = 2;
pub const SO_DEBUG: ::c_int = 3;
pub const SO_DONTROUTE: ::c_int = 4;
pub const SO_ERROR: ::c_int = 5;
pub const SO_KEEPALIVE: ::c_int = 6;
pub const SO_LINGER: ::c_int = 7;
pub const SO_OOBINLINE: ::c_int = 8;
pub const SO_RCVBUF: ::c_int = 9;
pub const SO_RCVLOWAT: ::c_int = 10;
pub const SO_RCVTIMEO: ::c_int = 11;
pub const SO_REUSEADDR: ::c_int = 12;
pub const SO_SNDBUF: ::c_int = 13;
pub const SO_SNDLOWAT: ::c_int = 14;
pub const SO_SNDTIMEO: ::c_int = 15;
pub const SO_TYPE: ::c_int = 16;
pub const SO_SNDBUFFORCE: ::c_int = 17;
pub const SO_PEERCRED: ::c_int = 18;
pub const SO_ATTACH_FILTER: ::c_int = 19;
pub const SO_PASSCRED: ::c_int = 20;
pub const SO_RCVBUFFORCE: ::c_int = 21;
pub const SO_DETACH_FILTER: ::c_int = 22;
pub const SO_PROTOCOL: ::c_int = 23;
pub const SO_REUSEPORT: ::c_int = 24;
pub const SO_TIMESTAMP: ::c_int = 25;

pub const SOMAXCONN: ::c_int = 1;

pub const MSG_CTRUNC: ::c_int = 0x1;
pub const MSG_DONTROUTE: ::c_int = 0x2;
pub const MSG_EOR: ::c_int = 0x4;
pub const MSG_OOB: ::c_int = 0x8;
pub const MSG_NOSIGNAL: ::c_int = 0x10;
pub const MSG_PEEK: ::c_int = 0x20;
pub const MSG_TRUNC: ::c_int = 0x40;
pub const MSG_WAITALL: ::c_int = 0x80;
pub const MSG_CONFIRM: ::c_int = 0x800;

pub const PF_INET: ::c_int = 1;
pub const PF_INET6: ::c_int = 2;
pub const PF_UNIX: ::c_int = 3;
pub const PF_LOCAL: ::c_int = 3;
pub const PF_UNSPEC: ::c_int = 4;
pub const PF_NETLINK: ::c_int = 5;
pub const PF_BRIDGE: ::c_int = 6;
pub const PF_APPLETALK: ::c_int = 7;
pub const PF_BLUETOOTH: ::c_int = 8;
pub const PF_DECnet: ::c_int = 9;
pub const PF_IPX: ::c_int = 10;
pub const PF_ISDN: ::c_int = 11;
pub const PF_SNA: ::c_int = 12;
pub const PF_PACKET: ::c_int = 13;

pub const AF_INET: ::c_int = PF_INET;
pub const AF_INET6: ::c_int = PF_INET6;
pub const AF_UNIX: ::c_int = PF_UNIX;
pub const AF_LOCAL: ::c_int = PF_LOCAL;
pub const AF_UNSPEC: ::c_int = PF_UNSPEC;
pub const AF_NETLINK: ::c_int = PF_NETLINK;
pub const AF_BRIDGE: ::c_int = PF_BRIDGE;
pub const AF_PACKET: ::c_int = PF_PACKET;

pub const SHUT_RD: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;
pub const SHUT_WR: ::c_int = 3;

pub type sa_family_t = ::c_uint;

extern "C" {
	#[no_mangle]
	pub fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int;

	#[no_mangle]
	pub fn recvfrom(
		socket: ::c_int,
		buf: *mut ::c_void,
		len: ::size_t,
		flags: ::c_int,
		addr: *mut ::sockaddr,
		addrlen: *mut ::socklen_t,
	) -> ::ssize_t;
}

#[repr(C)]
pub struct sockaddr_storage {
	pub ss_family: sa_family_t,
	padding: [u8; 128 - ::core::mem::size_of::<sa_family_t>()],
};
