// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


/// Logical cores to use for process and thread affinity, abstracting away platform, library and Operating System differences.
///
/// Provides the following definitions:-
///
/// * Constant `LogicalCores::IsSettingProcessAffinitySupported`: Boolean that is true if the current platform contains a way to attempt to set process affinity.
/// * Constant `LogicalCores::IsSettingThreadAffinitySupported`: Boolean that is true if the current platform contains a way to attempt to set thread affinity. Note that this is true for Fuschia and Empscripten, but that setting thread affinity always fails as unsupported as of 3rd December 2018.
///
/// * Method `set_current_process_affinity(&self) -> io::Result<()>`: Tries to set the current process' affinity; may fail (eg may not be supported in particular revisions of an OS, particularly so on Fuschia and Emscripten).
/// * Method `set_process_affinity(&self, process_identifier) -> io::Result<()>`: As above, but for a particular process. Usually fails with permission denied if not root.
/// * Method `set_current_thread_affinity(&self) -> io::Result<()>`: Tries to set the thread process' affinity; may fail (eg may not be supported in particular revisions of an OS, particularly so on Fuschia and Emscripten).
/// * Method `set_thread_affinity(&self, thread_identifier) -> io::Result<()>`: As above, but for a thread process. Usually fails with permission denied if not root.
///
/// If support for a platform has not been explicitly added to this library then it will not fail on that platform but it will not change any process or thread affinities, either.
///
/// At this time, there is no support for Solaris.
/// If you're interested in adding support, see <https://stackoverflow.com/questions/14085515/obtain-lwp-id-from-a-pthread-t-on-solaris-to-use-with-processor-bind>.
///
/// DragonFlyBSD and FreeBSD currently (as of 3rd December 2018) support 256 cores.
///
/// Windows is a bit squiffy with more than 64 cores.
///
/// Create using one of the `From` implementations.
///
/// Sadly, actually getting a list of the cores the current process can use is quite tricky; a little more Linux specific information can be obtained using the `dpdk-unix` crate's `HyperThread` struct and the `libnuma-sys` crate's static field `numa_all_cpus_ptr`, which is derived from the parsing of the line starting `Cpus_allowed:` in `/proc/self/status` and capping it with the maximum CPUs in the system. Yuck!
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalCores(HashSet<LogicalCoreIdentifier>);

impl From<LogicalCoreIdentifier> for LogicalCores
{
	/// From a core index.
	#[inline(always)]
	fn from(core_index: LogicalCoreIdentifier) -> Self
	{
		let mut logical_cores = HashSet::with_capacity(1);
		logical_cores.insert(core_index);
		LogicalCores(logical_cores)
	}
}

impl From<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn from(logical_cores: HashSet<LogicalCoreIdentifier>) -> Self
	{
		debug_assert_ne!(logical_cores.len(), 0, "There must be at least one logical core specified");

		LogicalCores(logical_cores)
	}
}

impl Into<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn into(self) -> HashSet<LogicalCoreIdentifier>
	{
		self.0
	}
}

impl Deref for LogicalCores
{
	type Target = HashSet<LogicalCoreIdentifier>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for LogicalCores
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl AsRef<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn as_ref(&self) -> &HashSet<LogicalCoreIdentifier>
	{
		&self.0
	}
}

impl AsMut<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut HashSet<LogicalCoreIdentifier>
	{
		&mut self.0
	}
}

impl Borrow<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn borrow(&self) -> &HashSet<LogicalCoreIdentifier>
	{
		&self.0
	}
}

impl BorrowMut<HashSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut HashSet<LogicalCoreIdentifier>
	{
		&mut self.0
	}
}

impl LogicalCores
{
	/// Is setting process affinity is supported?
	///
	/// Note that on emscripten and fuschia an error (`ENOSYS`) by the platform will always be returned as of the 3rd December 2018.
	///
	/// The following do not support setting process affinity:-
	///
	/// * iOS
	/// * Mac OS
	/// * OpenBSD
	/// * BitRig
	pub const IsSettingProcessAffinitySupported: bool = Self::_IsSettingProcessAffinitySupported;

	/// Is setting thread affinity is supported?
	///
	/// Note that on emscripten and fuschia an error (`ENOSYS`) by the platform will always be returned as of the 3rd December 2018.
	///
	/// The following do not support setting thread affinity:-
	///
	/// * Android
	/// * iOS
	/// * Mac OS
	/// * OpenBSD
	/// * BitRig
	pub const IsSettingThreadAffinitySupported: bool = Self::_IsSettingThreadAffinitySupported;

	/// Sets the current process' logical core affinity.
	///
	/// This is not efficiently implemented (it uses a loop).
	///
	/// Failure occurs if a CPU in the set does not exist, is offline or in some other way is unavailable to the `process_identifier` (`EINVAL`).
	#[inline(always)]
	pub fn set_current_process_affinity(&self) -> io::Result<()>
	{
		self._set_process_affinity(Self::current_process_identifier())
	}

	/// Sets the process' logical core affinity.
	///
	/// This is not efficiently implemented (it uses a loop).
	///
	/// Failure occurs if:-
	///
	/// * Permission is denied to change the process affinity for `process_identifier` (`EPERM`) (for example, the process isn't a child of this process);
	/// * A CPU in the set does not exist, is offline or in some other way is unavailable to the `process_identifier` (`EINVAL`);
	/// * `process_identifier` does not exist (`ESRCH`)
	/// * Operating system is not yet implemented (`ENOSYS`) - typical of Emscripten and Fuschia.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		self._set_process_affinity(process_identifier)
	}

	/// Sets the thread's logical core affinity.
	///
	/// Threads are never resident on just on core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> io::Result<()>
	{
		self._set_thread_affinity(Self::current_thread_identifier())
	}

	/// Sets the thread's logical core affinity.
	///
	/// This is only used as a *hint* on iOS and MacOS; it is near useless.
	///
	/// Threads are never resident on just on core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless.
	///
	/// * Permission is denied to change the process affinity for `process_identifier` (`EPERM`) (for example, the process isn't a child of this process);
	/// * A CPU in the set does not exist, is offline or in some other way is unavailable to the `process_identifier` (`EINVAL`);
	/// * `process_identifier` does not exist (`ESRCH`)
	/// * Operating system is not yet implemented (`ENOSYS`) - typical of Emscripten and Fuschia.
	/// * `ERANGE` - on FreeBSD, the cpu set was far too large.
	/// * `EDEADLK` - on FreeBSD, the cpu set could not be honoured.
	#[inline(always)]
	pub fn set_thread_affinity(&self, thread_identifier: ThreadIdentifier) -> io::Result<()>
	{
		self._set_thread_affinity(thread_identifier)
	}

	#[cfg(unix)]
	const fn current_process_identifier() -> ProcessIdentifier
	{
		0
	}

	#[cfg(windows)]
	fn current_process_identifier() -> ProcessIdentifier
	{
		unsafe { ::kernel32::GetCurrentProcess() }
	}

	#[cfg(unix)]
	fn current_thread_identifier() -> ThreadIdentifier
	{
		unsafe { pthread_self() }
	}

	#[cfg(windows)]
	fn current_thread_identifier() -> ThreadIdentifier
	{
		unsafe { ::kernel32::GetCurrentThread() }
	}

	#[allow(dead_code)]
	#[inline(always)]
	fn last_os_error() -> io::Error
	{
		io::Error::last_os_error()
	}
}

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_env = "uclibc"))] include!("LogicalCores.android-emscripten-fuschia-linux-uclibc.rs");
#[cfg(target_os = "dragonfly")] include!("LogicalCores.dragonfly.rs");
#[cfg(target_os = "freebsd")] include!("LogicalCores.freebsd.rs");
#[cfg(any(target_os = "ios", target_os = "macos"))] include!("LogicalCores.ios_macos.rs");
#[cfg(target_os = "netbsd")] include!("LogicalCores.netbsd.rs");
#[cfg(not(any(target_os = "android", target_os = "dragonfly", target_os = "emscripten", target_os = "freebsd", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "netbsd", target_env = "uclibc", windows)))] include!("LogicalCores.others.rs");
#[cfg(windows)] include!("LogicalCores.windows.rs");
