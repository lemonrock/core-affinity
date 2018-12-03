// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use self::freebsd::*;


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = false;

	const _IsSettingThreadAffinitySupported: bool = true;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		Ok(())
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		let cpu_set = self.as_cpuset_t()?;
		let result = unsafe { pthread_setaffinity_np(thread_identifier, ::std::mem::size_of(cpuset_t), &cpu_set) };
		let result = if result == 0
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		};
		result
	}

	#[inline(always)]
	fn as_cpuset_t(&self) -> cpuset_t
	{
		let mut cpu_set = Self::empty_cpuset_t();
		for logical_core in self.0.iter()
		{
			CPU_SET(n, &mut cpu_set);
		}
		Ok(cpu_set)
	}

	#[inline(always)]
	fn empty_cpuset_t() -> cpuset_t
	{
		unsafe { ::std::mem::zeroed() }
	}
}
