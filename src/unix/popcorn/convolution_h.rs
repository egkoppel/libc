extern "C" {
	#[no_mangle]
	pub fn convolution_get_current_exe(buffer: *mut ::c_char) -> ::size_t;
}