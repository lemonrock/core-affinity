// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


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


#[cfg(windows)] extern crate kernel32;
#[cfg(unix)] extern crate libc;
#[cfg(any(target_os = "ios", target_os = "macos"))] extern crate mach;
#[cfg(windows)] extern crate winapi;


#[cfg(unix)] use ::libc::pid_t;
#[cfg(unix)] use ::libc::pthread_self;
#[cfg(unix)] use ::libc::pthread_t;
use ::std::collections::HashSet;
use ::std::io;


#[cfg(target_os = "dragonfly")] pub(crate) mod dragonfly;


#[cfg(target_os = "emscripten")] pub(crate) mod emscripten;


#[cfg(target_os = "freebsd")] pub(crate) mod freebsd;


#[cfg(target_os = "fuschia")] pub(crate) mod fuschia;


#[cfg(any(target_os = "ios", target_os = "macos"))] pub(crate) mod ios_macos;


#[cfg(target_os = "netbsd")] pub(crate) mod netbsd;


#[cfg(target_env = "uclibc")] pub(crate) mod uclibc;


include!("LogicalCores.rs");
include!("ProcessIdentifier.rs");
include!("ThreadIdentifier.rs");
