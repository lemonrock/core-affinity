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
pub struct LogicalCores(BTreeSet<LogicalCoreIdentifier>);

impl From<LogicalCoreIdentifier> for LogicalCores
{
	/// From a core index.
	#[inline(always)]
	fn from(core_index: LogicalCoreIdentifier) -> Self
	{
		let mut logical_cores = BTreeSet::new();
		logical_cores.insert(core_index);
		Self(logical_cores)
	}
}

#[cfg(any(target_os = "android", target_os = "linux"))]
impl From<HyperThread> for LogicalCores
{
	/// From a core index.
	#[inline(always)]
	fn from(hyper_thread: HyperThread) -> Self
	{
		let logical_core_identifier: LogicalCoreIdentifier = hyper_thread.into();
		Self::from(logical_core_identifier)
	}
}

impl From<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn from(logical_cores: BTreeSet<LogicalCoreIdentifier>) -> Self
	{
		Self(logical_cores)
	}
}

#[cfg(any(target_os = "android", target_os = "linux"))]
impl From<BTreeSet<HyperThread>> for LogicalCores
{
	#[inline(always)]
	fn from(hyper_threads: BTreeSet<HyperThread>) -> Self
	{
		unsafe { transmute(hyper_threads) }
	}
}

impl Into<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn into(self) -> BTreeSet<LogicalCoreIdentifier>
	{
		self.0
	}
}

impl Deref for LogicalCores
{
	type Target = BTreeSet<LogicalCoreIdentifier>;

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

impl AsRef<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn as_ref(&self) -> &BTreeSet<LogicalCoreIdentifier>
	{
		&self.0
	}
}

impl AsMut<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut BTreeSet<LogicalCoreIdentifier>
	{
		&mut self.0
	}
}

impl Borrow<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn borrow(&self) -> &BTreeSet<LogicalCoreIdentifier>
	{
		&self.0
	}
}

impl BorrowMut<BTreeSet<LogicalCoreIdentifier>> for LogicalCores
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut BTreeSet<LogicalCoreIdentifier>
	{
		&mut self.0
	}
}

impl LogicalCores
{
	/// Valid logical cores for the current process.
	///
	/// ***Only valid at start up before `sched_setaffinity()` has been called.***
	///
	/// Logic inspired by [libnuma](https://github.com/numactl/numactl)'s `numa_num_task_cpus()` function.
	///
	/// Slow as it will parse the file `/proc/self/status`.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	pub fn valid_logical_cores_for_the_current_process() -> Self
	{
		Self::from(HyperThread::valid_hyper_threads_for_the_current_process(&ProcPath::default()))
	}

	/// Creates an empty set of per logical core data.
	#[inline(always)]
	pub fn empty_per_logical_core_data<PerLogicalCore>(&self) -> PerLogicalCoreData<PerLogicalCore>
	{
		PerLogicalCoreData::empty(self)
	}

	/// Creates a populated set of per logical core data.
	#[inline(always)]
	pub fn populate_per_logical_core_data<PerLogicalCore>(&self, constructor: impl FnMut(LogicalCoreIdentifier) -> PerLogicalCore) -> PerLogicalCoreData<PerLogicalCore>
	{
		PerLogicalCoreData::new(self, constructor)
	}

	/// Sets thread affinity to just the `logical_core_identifier`.
	///
	/// Not the same as `pthread_sched_getaffinity()`.
	#[inline(always)]
	pub fn set_current_thread_affinity_for_only_logical_core(logical_core_identifier: LogicalCoreIdentifier) -> Result<(), io::Error>
	{
		Self::from(logical_core_identifier).set_current_thread_affinity()
	}

	/// Set of logical cores (1) for the current thread.
	///
	/// Not the same as `pthread_sched_getaffinity()`.
	#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
	#[inline(always)]
	pub fn for_current_logical_core() -> Self
	{
		Self::from(Self::current_logical_core())
	}

	/// Logical core identifier for the current thread.
	#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
	#[inline(always)]
	pub fn current_logical_core() -> LogicalCoreIdentifier
	{
		HyperThread::current_hyper_thread().into()
	}

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
	/// * Operating system support is not yet implemented (`ENOSYS`) - typical of Emscripten and Fuschia.
	#[inline(always)]
	pub fn set_process_affinity(&self, process_identifier: ProcessIdentifier) -> io::Result<()>
	{
		self._set_process_affinity(process_identifier)
	}

	/// Sets the thread's logical core affinity.
	///
	/// Threads are never normally resident on just one core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless.
	#[inline(always)]
	pub fn set_current_thread_affinity(&self) -> io::Result<()>
	{
		self._set_thread_affinity(Self::current_thread_identifier())
	}

	/// Sets the thread's logical core affinity.
	///
	/// This is only used as a *hint* on iOS and MacOS; it is near useless on those platforms (Threads are never resident on just on core, and hence a lot of thread local opimizations (eg with clever non-blocking alogorithms) are useless).
	///
	/// Failure occurs if:-
	///
	/// * Permission is denied to change the process affinity for `process_identifier` (`EPERM`) (for example, the process isn't a child of this process);
	/// * A CPU in the set does not exist, is offline or in some other way is unavailable to the `process_identifier` (`EINVAL`);
	/// * `process_identifier` does not exist (`ESRCH`)
	/// * Operating system support is not yet implemented (`ENOSYS`) - typical of Emscripten and Fuschia.
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
