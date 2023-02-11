#[repr(C)]
pub struct sockaddr {
    pub sa_family: ::sa_family_t,
    pub sa_data: [::c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
	pub sin_family: ::sa_family_t;
	pub sin_port: ::in_port_t;
	pub sin_addr: ::in_addr;
	pad[u8; 8];
};

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
	pub s_addr: ::in_addr_t;
};
