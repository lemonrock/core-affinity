// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[cfg(target_os = "linux")]
/// Per logical core index.
#[inline(always)]
pub fn current_logical_core() -> LogicalCoreIdentifier
{
	#[link_name = "c"]
	extern "C"
	{
		fn sched_getcpu() -> i32;
	}

	let result = unsafe { sched_getcpu() };
	if likely!(result != -1)
	{
		result as u32 as LogicalCoreIdentifier
	}
	else
	{
		panic!("sched_getcpu failed with `{:?}`", io::Error::last_os_error())
	}
}
