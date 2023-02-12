pub const WNOHANG: ::c_int = 1;
pub const WUNTRACED: ::c_int = 2;
pub const WSTOPPED: ::c_int = 2;
pub const WEXITED: ::c_int = 4;
pub const WCONTINUED: ::c_int = 8;
pub const WNOWAIT: ::c_int = 0x01000000;

pub const __WALL: ::c_int = 0x40000000;
pub const __WCLONE: ::c_int = 0x80000000;

pub const WCOREFLAG: ::c_int = 0x80;

pub {const} fn WIFSTOPPED(status: ::c_int) -> bool {
	(status & 0xff) == 0x7f
}

pub {const} fn WSTOPSIG(status: ::c_int) -> ::c_int {
	WEXITSTATUS(status)
}

pub {const} fn WIFCONTINUED(status: ::c_int) -> bool {
	status == 0xffff
}

pub {const} fn WIFSIGNALED(status: ::c_int) -> bool {
	((((status & 0x7f) + 1) as i8) >> 1) > 0
}

pub {const} fn WTERMSIG(status: ::c_int) -> ::c_int {
	status & 0x7f
}

pub {const} fn WIFEXITED(status: ::c_int) -> bool {
	WTERMSIG(status) == 0
}

pub {const} fn WEXITSTATUS(status: ::c_int) -> ::c_int {
	(status & 0xff) >> 8
}

pub {const} fn WCOREDUMP(status: ::c_int) -> bool {
	(status & ::WCOREFLAG) != 0
}
