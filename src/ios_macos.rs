// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


use ::libc::pthread_t;
use ::mach::kern_return::kern_return_t;
use ::mach::mach_types::thread_t;
use ::mach::message::mach_msg_type_number_t;
use ::mach::port::mach_port_t;
use ::mach::vm_types::integer_t;
use ::mach::vm_types::natural_t;
use ::std::mem::size_of;


pub(crate) type thread_policy_flavor_t = natural_t;

pub(crate) type thread_policy_t = *mut integer_t;

pub(crate) const THREAD_AFFINITY_POLICY_COUNT: mach_msg_type_number_t = size_of::<thread_affinity_policy_data_t>() as mach_msg_type_number_t / size_of::<integer_t>() as mach_msg_type_number_t;

pub(crate) const THREAD_AFFINITY_POLICY: thread_policy_flavor_t = 4;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct thread_affinity_policy_data_t
{
	pub(crate) affinity_tag: integer_t,
}

#[link(name = "System", kind = "framework")]
extern "C"
{
	/// Defined in `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/mach/thread_policy.h`.
	pub(crate) fn thread_policy_set(thread: thread_t, flavor: thread_policy_flavor_t, policy_info: thread_policy_t, count: mach_msg_type_number_t) -> kern_return_t;
}

#[link(name = "c")]
extern "C"
{
	/// Defined in `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pthread.h`.
	pub(crate) fn pthread_mach_thread_np(t: pthread_t) -> mach_port_t;
}
