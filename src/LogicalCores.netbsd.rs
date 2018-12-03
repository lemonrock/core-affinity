// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use self::netbsd::*;


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = true;

	const _IsSettingThreadAffinitySupported: bool = true;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		let cpu_set = self.as_cpuset_t()?;
		let result = match unsafe { sched_setaffinity_np(process_identifier, cpuset_size(cpu_set), cpu_set) }
		{
			0 => Ok(()),
			-1 => Err(Self::last_os_error()),
			_ => unreachable!(),
		};
		unsafe { cpuset_destroy(cpu_set) };
		result
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		let cpu_set = self.as_cpuset_t()?;
		let result = unsafe { pthread_setaffinity_np(thread_identifier, cpuset_size(cpu_set), cpu_set) };
		let result = if result == 0
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		};
		unsafe { cpuset_destroy(cpu_set) };
		result
	}

	#[inline(always)]
	fn as_cpuset_t(&self) -> io::Result<*mut cpuset_t>
	{
		let mut cpu_set = Self::empty_cpuset_t()?;
		for logical_core in self.0.iter()
		{
			unsafe { cpuset_set(*(logical_core as u32), cpu_set) };
		}
		Ok(cpu_set)
	}

	#[inline(always)]
	fn empty_cpuset_t() -> io::Result<*mut cpuset_t>
	{
		let mut cpu_set = unsafe { cpuset_create() };
		if cpu_set.is_null()
		{
			Err(Err(io::Error::from(io::ErrorKind::Other)))
		}
		else
		{
			Ok(cpu_set)
		}
	}
}
