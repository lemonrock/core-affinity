// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.



use ::libc::c_int;
use ::libc::pthread_t;
use ::libc::size_t;


/// Defined in `sys/cpu/x86_64/include/types.h` in DragonFlyBSD source code.
pub(crate) struct cpumask_t
{
	pub(crate) ary: [u64; 4],
}

/// Defined in `sys/sys/sched.h` in DragonFlyBSD source code.
pub(crate) type cpu_set_t = cpumask_t;


/// Defined in `sys/cpu/x86_64/include/cpumask.h` in DragonFlyBSD source code.
pub(crate) const fn CPUMASK_SIMPLE(cpu: u64) -> u64
{
	1 << cpu
}

/// Defined in `sys/cpu/x86_64/include/cpumask.h` in DragonFlyBSD source code.
pub(crate) const fn CPUMASK_ORBIT(mask: &mut cpumask_t, i: u64)
{
	mask.ary[(i >> 6) & 3] |= CPUMASK_SIMPLE(i & 63)
}

/// Defined in `sys/sys/sched.h` in DragonFlyBSD source code.
pub(crate) const fn CPU_SET(set: &mut cpu_set_t, cpu: u64)
{
	CPUMASK_ORBIT(set, cpu)
}

extern "C"
{
	/// Defined in `sys/sys/sched.h` in the DragonFlyBSD source code.
	pub(crate) fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;

	/// Defined in `include/pthread_np.h` in the DragonFlyBSD source code.
	pub(crate) fn pthread_setaffinity_np(tid: pthread_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
}
