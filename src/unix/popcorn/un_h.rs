#[repr(C)]
pub struct sockaddr_un {
	pub sun_family: ::sa_family_t,
	pub sun_path: [::c_char; 108],
}

impl core::clone::Clone for sockaddr_un {
	fn clone(&self) -> Self {
        *self
    }
}
impl core::marker::Copy for sockaddr_un {}
