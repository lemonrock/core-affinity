// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Data with an item per logical core in use by the process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerLogicalCoreData<PerLogicalCore>
{
	logical_cores_data: Box<[Option<PerLogicalCore>]>,
}

impl<PerLogicalCore> Deref for PerLogicalCoreData<PerLogicalCore>
{
	type Target = [Option<PerLogicalCore>];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.logical_cores_data
	}
}

impl<PerLogicalCore> DerefMut for PerLogicalCoreData<PerLogicalCore>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.logical_cores_data
	}
}

impl<PerLogicalCore> Index<LogicalCoreIdentifier> for PerLogicalCoreData<PerLogicalCore>
{
	type Output = Option<PerLogicalCore>;

	#[inline(always)]
	fn index(&self, index: LogicalCoreIdentifier) -> &Self::Output
	{
		self.logical_cores_data.index(index as usize)
	}
}

impl<PerLogicalCore> IndexMut<LogicalCoreIdentifier> for PerLogicalCoreData<PerLogicalCore>
{
	#[inline(always)]
	fn index_mut(&mut self, index: LogicalCoreIdentifier) -> &mut Self::Output
	{
		self.logical_cores_data.index_mut(index as usize)
	}
}

impl<PerLogicalCore> PerLogicalCoreData<PerLogicalCore>
{
	/// Creates an empty set of logical core data.
	#[inline(always)]
	pub fn empty(logical_cores: &LogicalCores) -> Self
	{
		let number_of_logical_cores = logical_cores.len();
		assert_ne!(number_of_logical_cores, 0, "Must be at least one logical core");

		let mut logical_cores_data = Vec::with_capacity(number_of_logical_cores);
		for _logical_core_index in 0 .. number_of_logical_cores
		{
			logical_cores_data.push(None);
		}

		Self
		{
			logical_cores_data: logical_cores_data.into_boxed_slice()
		}
	}

	/// `constructor` is called for each defined logical core in `logical_cores`; it is passed the logical core's identifier.
	#[inline(always)]
	pub fn new(logical_cores: &LogicalCores, mut constructor: impl FnMut(LogicalCoreIdentifier) -> PerLogicalCore) -> Self
	{
		Self::new_internal(logical_cores, |logical_core_identifier| Some(constructor(logical_core_identifier)))
	}

	#[inline(always)]
	fn new_internal(logical_cores: &LogicalCores, mut constructor: impl FnMut(LogicalCoreIdentifier) -> Option<PerLogicalCore>) -> Self
	{
		Self
		{
			logical_cores_data:
			{
				let number_of_logical_cores = logical_cores.len();
				assert_ne!(number_of_logical_cores, 0, "Must be at least one logical core");

				// Since the highest logical core is not necessarily the same as the length, this could still be resized.
				let mut logical_cores_data = Vec::with_capacity(number_of_logical_cores);
				let mut current_logical_core = 0;
				for logical_core_identifier_reference in logical_cores.iter()
				{
					let logical_core_identifier = *logical_core_identifier_reference;

					while current_logical_core < logical_core_identifier
					{
						logical_cores_data.push(None);
						current_logical_core += 1;
					}
					debug_assert_eq!(current_logical_core, logical_core_identifier);
					logical_cores_data.push(constructor(logical_core_identifier as LogicalCoreIdentifier));

					current_logical_core = logical_core_identifier + 1;
				}
				debug_assert_eq!(current_logical_core as usize, logical_cores_data.len());

				logical_cores_data.into_boxed_slice()
			},
		}
	}

	/// Gets the data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn get(&self, logical_core_identifier: LogicalCoreIdentifier) -> Option<&PerLogicalCore>
	{
		let logical_core_identifier = logical_core_identifier as usize;
		if unlikely!(logical_core_identifier >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked(logical_core_identifier).as_ref() }
	}

	/// Gets the data for a particular logical core; if no data for that core, gets it for `LogicalCore::current_logical_core()`.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn get_or_current(&mut self, logical_core_identifier: LogicalCoreIdentifier) -> &PerLogicalCore
	{
		self.get_or(logical_core_identifier, LogicalCores::current_logical_core)
	}

	/// Gets the data for a particular logical core; if no data for that core, gets it for the `default_logical_core_identifier`.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[inline(always)]
	pub fn get_or(&mut self, logical_core_identifier: LogicalCoreIdentifier, default_logical_core_identifier: impl FnOnce() -> LogicalCoreIdentifier) -> &PerLogicalCore
	{
		let logical_core_identifier = if unlikely!(self.get(logical_core_identifier).is_none())
		{
			default_logical_core_identifier() as usize
		}
		else
		{
			logical_core_identifier as usize
		};

		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_identifier).as_mut().unwrap() }
	}

	/// Gets the mutable data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[inline(always)]
	pub fn get_mut(&mut self, logical_core_identifier: LogicalCoreIdentifier) -> Option<&mut PerLogicalCore>
	{
		let logical_core_identifier = logical_core_identifier as usize;
		if unlikely!(logical_core_identifier >= self.logical_cores_data.len())
		{
			return None
		}
		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_identifier).as_mut() }
	}

	/// Gets the mutable_data for a particular logical core; if no data for that core, gets it for `LogicalCore::current_logical_core()`.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	#[inline(always)]
	pub fn get_mut_or_current(&mut self, logical_core_identifier: LogicalCoreIdentifier) -> &mut PerLogicalCore
	{
		self.get_mut_or(logical_core_identifier, LogicalCores::current_logical_core)
	}

	/// Gets the mutable data for a particular logical core; if no data for that core, gets it for the `default_logical_core_identifier`.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the` SO_INCOMING_CPU` socket option, which can return an index for a CPU not assigned to the process.
	#[inline(always)]
	pub fn get_mut_or(&mut self, logical_core_identifier: LogicalCoreIdentifier, default_logical_core_identifier: impl FnOnce() -> LogicalCoreIdentifier) -> &mut PerLogicalCore
	{
		let logical_core_identifier = if unlikely!(self.get(logical_core_identifier).is_none())
		{
			default_logical_core_identifier() as usize
		}
		else
		{
			logical_core_identifier as usize
		};

		unsafe { self.logical_cores_data.get_unchecked_mut(logical_core_identifier).as_mut().unwrap() }
	}

	/// Sets the current value, discarding the old one.
	#[inline(always)]
	pub fn set(&mut self, logical_core_identifier: LogicalCoreIdentifier, value: PerLogicalCore)
	{
		self.logical_cores_data[logical_core_identifier as usize] = Some(value)
	}

	/// Takes the data for a particular logical core.
	///
	/// If the logical core does not exist (or does not have assigned data), returns None; this can happen on Linux if using the SO_INCOMING_CPU socket option, which can map to a CPU not assigned to the process.
	#[inline(always)]
	pub fn take(&mut self, logical_core_identifier: LogicalCoreIdentifier) -> Option<PerLogicalCore>
	{
		self.logical_cores_data[logical_core_identifier as usize].take()
	}

	/// Replaces the current value, returning the old one.
	#[inline(always)]
	pub fn replace(&mut self, logical_core_identifier: LogicalCoreIdentifier, value: PerLogicalCore) -> Option<PerLogicalCore>
	{
		self.logical_cores_data[logical_core_identifier as usize].replace(value)
	}

	/// Iterates over all entries that are not `None`.
	#[inline(always)]
	pub fn logical_core_indices<'a>(&'a self) -> impl Iterator<Item=LogicalCoreIdentifier> + 'a
	{
		(0u16 .. self.logical_cores_data.len() as LogicalCoreIdentifier).into_iter().filter(move |potential_logical_core_index| self.get(*potential_logical_core_index).is_some())
	}

	/// Maps from `T` to `V` assuming that entries with `Some()` in them are mappable.
	///
	/// `mapper` is called for each defined (ie is Some(T)) logical core in `logical_cores`; it is passed the logical core's identifier and the old value.
	#[inline(always)]
	pub fn map<V>(mut self, mut mapper: impl FnMut(LogicalCoreIdentifier, PerLogicalCore) -> V) -> PerLogicalCoreData<V>
	{
		let number_of_logical_cores = self.logical_cores_data.len();

		let mut mapped_logical_cores_data = Vec::with_capacity(number_of_logical_cores);

		for logical_core_index in 0 .. number_of_logical_cores as LogicalCoreIdentifier
		{
			let v_option = match self.take(logical_core_index)
			{
				None => None,
				Some(t) => Some(mapper(logical_core_index as LogicalCoreIdentifier, t)),
			};
			mapped_logical_cores_data.push(v_option);
		}

		PerLogicalCoreData
		{
			logical_cores_data: mapped_logical_cores_data.into_boxed_slice()
		}
	}
}
