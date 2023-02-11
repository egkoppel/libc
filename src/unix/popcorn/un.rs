pub struct sockaddr_un {
	pub sun_family: ::sa_family_t,
	pub sun_path: [::c_char; 108],
}
