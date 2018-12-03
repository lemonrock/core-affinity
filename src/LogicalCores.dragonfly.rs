// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.



#[cfg(target_os = "dragonfly")] use self::dragonfly::*;
use ::std::mem::size_of;
use ::std::mem::zeroed;


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = true;

	const _IsSettingThreadAffinitySupported: bool = true;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		let cpu_set = self.as_cpu_set_t();
		let result = unsafe { sched_setaffinity(process_identifier, Self::SizeOfCpuSetT, cpu_set) };
		if result == 0
		{
			Ok(())
		}
		else
		{
			Err(Self::last_os_error())
		}
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		let cpu_set = self.as_cpu_set_t();
		let result = unsafe { pthread_setaffinity_np(thread_identifier, Self::SizeOfCpuSetT, cpu_set) };
		if result == 0
		{
			Ok(())
		}
		else
		{
			Err(io::Error::from_raw_os_error(result))
		}
	}

	#[inline(always)]
	fn as_cpu_set_t(&self) -> cpu_set_t
	{
		let mut cpu_set_t = Self::empty_cpu_set_t();
		for logical_core in self.0.iter()
		{
			unsafe { CPU_SET(&mut cpu_set_t, (*logical_core) as u64) };
		}
		cpu_set_t
	}

	#[inline(always)]
	fn empty_cpu_set_t() -> cpu_set_t
	{
		unsafe { zeroed() }
	}

	const SizeOfCpuSetT: usize = size_of::<cpu_set_t>();
}
