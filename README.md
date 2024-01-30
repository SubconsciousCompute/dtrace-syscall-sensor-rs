# dtrace-syscall-sensor

## Description

A DTrace-based consumer to get information about system calls on windows.

## Getting Started

Make sure you have the prerequisites for [libdtrace-rs](https://github.com/SubconsciousCompute/libdtrace-rs) setup.

You need to place [`dtrace.dll`](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/dtrace) file inside the executable directory manually. It can be found at `./target/$PROFILE/build/libdtrace_rs-<hash>/out/dtrace.dll`

### Executing program

Open PowerShell or your favorite terminal in privileged mode, clone the repo and execute 

```shell
$ cargo run
```
