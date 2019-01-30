// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = true;

	#[cfg(any(target_os = "android"))] const _IsSettingThreadAffinitySupported: bool = false;
	#[cfg(any(target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_env = "uclibc"))] const _IsSettingThreadAffinitySupported: bool = false;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		self.to_cpu_set().set_process_affinity(process_identifier)
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		self.to_cpu_set().set_thread_affinity(thread_identifier)
	}

	#[inline(always)]
	fn to_cpu_set(&self) -> CpuSet
	{
		let mut cpu_set = CpuSet::default();
		for logical_core in self.0.iter()
		{
			cpu_set.set_hyper_thread(HyperThread::from(*logical_core));
		}
		cpu_set
	}
}
