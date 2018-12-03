// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::c_long;
use ::libc::pthread_t;
use ::libc::size_t;
use ::std::mem::size_of;


/// Defined in `/sys/sys/_bitset.h` in FreeBSD source code.
const _BITSET_BITS: usize = size_of::<c_long>() * 8;

/// Defined in `/sys/sys/_bitset.h` in FreeBSD source code.
#[inline(always)]
const fn __howmany(x: usize, y: usize) -> usize
{
	(x + (y - 1)) / y
}

/// Defined in `/sys/sys/_bitset.h` in FreeBSD source code.
#[inline(always)]
const fn __bitset_words(_s: usize) -> usize
{
	__howmany(_s, _BITSET_BITS)
}

/// Defined in `/sys/sys/_bitset.h` in FreeBSD source code.
macro_rules! BITSET_DEFINE
{
	($t: tt, $_s: ident) =>
	{
		pub(crate) type $t = [c_long; __bitset_words($_s)]
	}
}

/// Defined in `/sys/sys/bitset.h` in FreeBSD source code.
#[inline(always)]
fn __bitset_mask(_s: usize, n: size_t) -> c_long
{
	let relative_bit = if __bitset_words(_s) == 1
	{
		n
	}
	else
	{
		n % _BITSET_BITS
	};

	(1 << relative_bit) as c_long
}

/// Defined in `/sys/sys/bitset.h` in FreeBSD source code.
#[inline(always)]
fn __bitset_word(_s: usize, n: size_t) -> usize
{
	if __bitset_words(_s) == 1
	{
		0
	}
	else
	{
		n / _BITSET_BITS
	}
}

/// Defined in `/sys/sys/bitset.h` in FreeBSD source code.
#[inline(always)]
fn BIT_SET(_s: usize, n: size_t, p: &mut _cpuset)
{
	p.__bits[__bitset_word(_s, n)] |= __bitset_mask(_s, n)
}

/// Defined in `/sys/sys/_cpuset.h` in FreeBSD source code.
const CPU_MAXSIZE: usize = 256;

/// Defined in `/sys/sys/_cpuset.h` in FreeBSD source code.
const CPU_SETSIZE: usize = CPU_MAXSIZE;

/// Defined in `/sys/sys/_cpuset.h` in FreeBSD source code.
BITSET_DEFINE!(_cpuset, CPU_SETSIZE);

/// Defined in `/sys/sys/_cpuset.h` in FreeBSD source code.
pub(crate) type cpuset_t = _cpuset;

/// Defined in `/sys/sys/cpuset.h` in FreeBSD source code.
#[inline(always)]
pub(crate) const fn CPU_SET(n: usize, p: &mut _cpuset)
{
	BIT_SET(CPU_SETSIZE, n, p)
}

extern "C"
{
	/// Defined in `include/pthread_np.h` in FreeBSD source code.
	pub(crate) fn pthread_setaffinity_np(td: pthread_t, cpusetsize: size_t, cpusetp: *const cpuset_t) -> c_int;
}
