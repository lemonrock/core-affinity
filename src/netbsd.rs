// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::c_ulong;
use ::libc::pid_t;
use ::libc::pthread_t;
use ::libc::size_t;


/// Defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
pub(crate) struct _cpuset
{
	pub(crate) bits: [u32; 0],
}


/// Defined in `sys/sys/sched.h` in NetBSD source code.
pub(crate) type cpuset_t = _cpuset;


/// Deinfed in `sys/sys/types.h` in NetBSD source code.
pub(crate) type cpuid_t = c_ulong;


#[link(name = "c")]
extern "C"
{
	/// Process affinity.
	///
	/// Returns `0` on success and `-1` on error.
	///
	/// Sets `errno` on error.
	///
	/// Defined in `include/sched.h` in NetBSD source code.
	pub(crate) fn sched_setaffinity_np(p: pid_t, s: size_t, c: *mut cpuset_t) -> c_int;

	/// Thread affinity.
	///
	/// Returns `0` on success and an `ERR*` value on error.
	///
	/// The failures are listed as `EINVAL` (`c` was invalid), `EPERM` (permission denied) and `ESRCH` (`t` was not found).
	///
	/// The value of `s` is `cpuset_size(c)`.
	///
	/// Defined in `lib/libpthread/pthread.h` in NetBSD source code.
	pub(crate) fn pthread_setaffinity_np(t: pthread_t, s: size_t, c: *mut cpuset_t) -> c_int;

	/// Creates a cpu set that must be destroyed with `cpuset_destroy()`.
	///
	/// Returns a null pointer in the event of an error.
	///
	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	#[link_name = "_cpuset_create"]
	pub(crate) fn cpuset_create() -> *mut cpuset_t;

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	#[link_name = "_cpuset_destroy"]
	pub(crate) fn cpuset_destroy(c: *mut cpuset_t);

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	///
	/// Note that in NetBSD as of 3rd Dec 2018 the result of this function is constant.
	#[link_name = "_cpuset_size"]
	pub(crate) fn cpuset_size(c: *mut cpuset_t) -> size_t;

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	#[link_name = "_cpuset_zero"]
	pub(crate) fn cpuset_zero(c: *mut cpuset_t);

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	///
	/// Sets `errno` to `EINVAL` if `i` is too large and returns `-1`.
	/// Otherwise returns a non-zero value.
	#[link_name = "_cpuset_isset"]
	pub(crate) fn cpuset_isset(i: cpuid_t, c: *mut cpuset_t) -> c_int;

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	///
	/// Sets `errno` to `EINVAL` if `i` is too large and returns `-1`.
	/// Otherwise returns `0`.
	#[link_name = "_cpuset_set"]
	pub(crate) fn cpuset_set(i: cpuid_t, c: *mut cpuset_t) -> c_int;

	/// Public `#define` is in `sys/sys/sched.h` but actual function is defined in `common/lib/libc/sys/cpuset.c` in NetBSD source code.
	///
	/// Sets `errno` to `EINVAL` if `i` is too large and returns `-1`.
	/// Otherwise returns `0`.
	#[link_name = "_cpuset_clr"]
	pub(crate) fn cpuset_clr(i: cpuid_t, c: *mut cpuset_t) -> c_int;
}
