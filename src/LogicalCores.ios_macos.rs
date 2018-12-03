// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use self::ios_macos::*;
use ::mach::mach_types::thread_t;
use ::mach::vm_types::integer_t;
use ::mach::kern_return::KERN_SUCCESS;


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = false;

	const _IsSettingThreadAffinitySupported: bool = false;

	#[inline(always)]
	fn _set_process_affinity(&self, _process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		Ok(())
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, _thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		Ok(())
	}

	/// Sets the current thread's level 2 cache affinity tag hint.
	///
	/// This is only used as a *hint* on iOS and MacOS; it is near useless.
	///
	/// Threads are never resident on just on core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless.
	#[inline(always)]
	pub fn set_current_thread_affinity_hint(affinity_tag: usize) -> io::Result<()>
	{
		Self::set_thread_affinity_hint(Self::current_thread_identifier(), affinity_tag)
	}

	/// Sets the thread's level 2 cache affinity tag hint.
	///
	/// This is only used as a *hint* on iOS and MacOS; it is near useless.
	///
	/// Threads are never resident on just on core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless.
	#[inline(always)]
	pub fn set_thread_affinity_hint(thread_identifier: ThreadIdentifier, affinity_tag: usize) -> io::Result<()>
	{
		// /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/thread_policy.h:-
		// "Threads with the same affinity tag will be scheduled to share an L2 cache if possible.
		// That is, affinity tags are a hint to the scheduler for thread placement".
		let mut info = thread_affinity_policy_data_t
		{
			affinity_tag: affinity_tag as integer_t,
		};

		let thread = unsafe { pthread_mach_thread_np(thread_identifier) };
		match unsafe { thread_policy_set(thread as thread_t, THREAD_AFFINITY_POLICY, &mut info as *mut _ as thread_policy_t, THREAD_AFFINITY_POLICY_COUNT) }
		{
			KERN_SUCCESS => Ok(()),

			_ => Err(io::Error::from(io::ErrorKind::Other)),
		}
	}
}
