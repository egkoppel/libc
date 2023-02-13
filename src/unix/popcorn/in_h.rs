#[repr(C)]
pub struct sockaddr {
    pub sa_family: ::sa_family_t,
    pub sa_data: [::c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
	pub sin_family: ::sa_family_t,
	pub sin_port: ::in_port_t,
	pub sin_addr: ::in_addr,
	pad: [u8; 8],
}

impl core::clone::Clone for sockaddr_in {}
impl core::marker::Copy for sockaddr_in {}

#[repr(C)]
pub struct sockaddr_in6 {
	pub sin6_family: ::sa_family_t,
	pub sin6_port: ::in_port_t,
	pub sin6_flowinfo: u32,
	pub sin6_addr: ::in6_addr,
	pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct in_addr {
	pub s_addr: ::in_addr_t,
}

impl core::clone::Clone for in_addr {}
impl core::marker::Copy for in_addr {}

pub const IPV6_JOIN_GROUP: ::c_int = 1;
pub const IPV6_LEAVE_GROUP: ::c_int = 2;
pub const IPV6_MULTICAST_HOPS: ::c_int = 3;
pub const IPV6_MULTICAST_IF: ::c_int = 4;
pub const IPV6_MULTICAST_LOOP: ::c_int = 5;
pub const IPV6_UNICAST_HOPS: ::c_int = 6;
pub const IPV6_V6ONLY: ::c_int = 7;
pub const IPV6_PMTUDISC_DONT: ::c_int = 8;
pub const IPV6_PMTUDISC_DO: ::c_int = 9;
pub const IPV6_MTU_DISCOVER: ::c_int = 23;
pub const IPV6_RECVERR: ::c_int = 25;
pub const IPV6_RECVPKTINFO: ::c_int = 49;
pub const IPV6_PKTINFO: ::c_int = 50;
pub const IPV6_RECVHOPLIMIT: ::c_int = 51;
pub const IPV6_HOPLIMIT: ::c_int = 52;
pub const IPV6_TCLASS: ::c_int = 67;

pub const IP_TOS: ::c_int = 1;
pub const IP_TTL: ::c_int = 2;
pub const IP_OPTIONS: ::c_int = 4;
pub const IP_PKTINFO: ::c_int = 8;
pub const IP_MTU_DISCOVER: ::c_int = 10;
pub const IP_RECVERR: ::c_int = 11;
pub const IP_RECVTTL: ::c_int = 12;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int =  1;
pub const IP_MULTICAST_IF: ::c_int =           32;
pub const IP_MULTICAST_TTL: ::c_int =          33;
pub const IP_MULTICAST_LOOP: ::c_int =         34;
pub const IP_ADD_MEMBERSHIP: ::c_int =         35;
pub const IP_DROP_MEMBERSHIP: ::c_int =        36;
pub const IP_UNBLOCK_SOURCE: ::c_int =         37;
pub const IP_BLOCK_SOURCE: ::c_int =           38;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::c_int =  39;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::c_int = 40;
pub const MCAST_JOIN_SOURCE_GROUP: ::c_int =   46;
pub const MCAST_LEAVE_SOURCE_GROUP: ::c_int =  47;

pub const IP_PMTUDISC_DONT: ::c_int = 0;
pub const IP_PMTUDISC_DO: ::c_int =   2;

pub const IPV6_ADD_MEMBERSHIP: ::c_int =  IPV6_JOIN_GROUP;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = IPV6_LEAVE_GROUP;

pub struct ip_mreq {
	pub imr_multiaddr: in_addr,
	pub imr_interface: in_addr,
}
