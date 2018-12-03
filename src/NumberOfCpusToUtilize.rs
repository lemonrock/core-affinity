// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


/// Number of logical CPUs to utilize (ie count simultaneous multi-threads (SMT), also known as hyper-threads).
///
/// If not specified (None) will default to machine maximum.
///
/// If specified as 0 will default to 1.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumberOfCpusToUtilize(pub Option<usize>);

impl NumberOfCpusToUtilize
{
	#[inline(always)]
	pub(crate) fn number_of_worker_threads_and_first_core_index(&self) -> (usize, usize)
	{
		#[inline(always)]
		fn number_of_cpus_cap() -> usize
		{
			match num_cpus::get()
			{
				0 | 1 => 1,
				other @ _ => other - 1,
			}
		}

		let number_of_worker_threads = match self.0
		{
			Some(0) => 1,
			Some(maximum) => min(number_of_cpus_cap(), maximum),
			None => number_of_cpus_cap(),
		};

		if number_of_worker_threads == 1
		{
			(1, 0)
		}
		else
		{
			(number_of_worker_threads, 1)
		}
	}
}
