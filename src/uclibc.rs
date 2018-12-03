// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::cpu_set_t;
use ::libc::pthread_t;
use ::libc::size_t;


#[link(name = "c")]
extern "C"
{
	/// Defined in `libpthread/nptl/sysdeps/pthread/pthread.h` in uclibc-ng source code.
	pub(crate) fn pthread_setaffinity_np(__th: pthread_t, __cpusetsize: size_t, __cpuset: *const cpu_set_t) -> c_int;
}
