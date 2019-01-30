# cpu-affinity

CPU affinity for processes and threads across many platforms and Operating Systems, including:-

* Android
* BitRig (does nothing)
* DragonFlyBSD
* Emscripten
* Fuschia
* FreeBSD
* iOS
* MacOS (does nothing, but special logic for setting thread affinity groups)
* Linux
* NetBSD
* OpenBSD (does nothing)
* Windows

Provides logic for finding out the valid set of logical cores (hyper threads) for a process, managing per-logical-core data sets, and more.


## Licensing

The license for this project is MIT.
