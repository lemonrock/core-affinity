// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


impl LogicalCores
{
	const _IsSettingProcessAffinitySupported: bool = false;

	const _IsSettingThreadAffinitySupported: bool = false;

	#[inline(always)]
	fn _set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		Ok(())
	}

	#[inline(always)]
	fn _set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		Ok(())
	}
}
