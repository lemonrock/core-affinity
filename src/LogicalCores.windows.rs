// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::winapi::basetsd::DWORD_PTR;
use ::winapi::shared::ntdef::HANDLE;


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = true;

	const _IsSettingThreadAffinitySupported: bool = true;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		match unsafe { ::kernel32::SetProcessAffinityMask(process_identifier, self.as_mask()) }
		{
			0 => Err(Self::last_os_error()),
			_previous_affinity_mask @ _ => Ok(()),
		}
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		match unsafe { ::kernel32::SetThreadAffinityMask(thread_identifier, self.as_mask()) }
		{
			0 => Err(Self::last_os_error()),
			_previous_affinity_mask @ _ => Ok(()),
		}
	}

	#[inline(always)]
	fn as_mask(&self) -> DWORD_PTR
	{
		let mut mask = 0;

		for logical_core in self.0.iter()
		{
			mask |= 1 << ((*logical_core) as DWORD_PTR)
		}

		mask
	}

//	/// Gets the current process' logical core affinity.
//	///
//	/// This is not efficiently implemented (it uses a loop).
//	#[inline(always)]
//	fn get_current_process_affinity() -> io::Result<Self>
//	{
//		let mut process_affinity_mask = unsafe { uninitialized() };
//		let mut system_affinity_mask = unsafe { uninitialized() };
//
//		match unsafe { GetProcessAffinityMask(GetCurrentProcess(),&mut process_affinity_mask, &mut system_affinity_mask) }
//		{
//			0 => Err(Self::last_os_error()),
//			_ =>
//			{
//				let mut set = HashSet::with_capacity(64);
//
//				for index in 0 .. 64
//				{
//					if index | (1 << index) != 0
//					{
//						set.insert(index);
//					}
//				}
//
//				set.shrink_to_fit();
//
//				Ok(LogicalCores(set))
//			}
//		}
//	}
}
