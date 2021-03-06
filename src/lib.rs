// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(core_intrinsics)]


//! # cpu-affinity
//!
//! CPU affinity for processes and threads across many platforms and Operating Systems, including:-
//!
//! * Android
//! * BitRig (does nothing)
//! * DragonFlyBSD
//! * Emscripten
//! * Fuschia
//! * FreeBSD
//! * iOS
//! * MacOS (does nothing, but special logic for setting thread affinity groups)
//! * Linux
//! * NetBSD
//! * OpenBSD (does nothing)
//! * Windows
//! * uclibc


#[cfg(any(target_os = "android", target_os = "linux"))] extern crate dpdk_unix;
#[cfg(windows)] extern crate kernel32;
#[cfg(unix)] extern crate libc;
#[macro_use] extern crate likely;
#[cfg(any(target_os = "ios", target_os = "macos"))] extern crate mach;
#[cfg(windows)] extern crate winapi;


#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::ProcPath;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::scheduling::CpuSet;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::dpdk_unix::hyper_thread::HyperThread;
#[cfg(unix)] use ::libc::pid_t;
#[cfg(unix)] use ::libc::pthread_self;
#[cfg(unix)] use ::libc::pthread_t;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::collections::BTreeSet;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ops::Index;
use ::std::ops::IndexMut;
use ::std::io;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::mem::transmute;


#[cfg(target_os = "dragonfly")] pub(crate) mod dragonfly;


#[cfg(target_os = "freebsd")] pub(crate) mod freebsd;


#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) mod ios_macos;


#[cfg(target_os = "netbsd")] pub(crate) mod netbsd;


#[cfg(target_env = "uclibc")] pub(crate) mod uclibc;


include!("LogicalCores.rs");
include!("LogicalCoreIdentifier.rs");
include!("PerLogicalCoreData.rs");
include!("ProcessIdentifier.rs");
include!("ThreadIdentifier.rs");
