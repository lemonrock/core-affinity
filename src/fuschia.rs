// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::libc::pthread_t;
use ::libc::size_t;
use ::libc::cpuset_t;


#[link(name = "c")]
extern "C"
{
	/// Thread affinity.
	///
	/// Returns `0` on success and an `ERR*` value on error.
	///
	/// AS of 3rd December 2018 always returns `ENOSYS`.
	pub(crate) fn pthread_setaffinity_np(td: pthread_t, size: size_t, c: *const cpuset_t) -> c_int;
}
