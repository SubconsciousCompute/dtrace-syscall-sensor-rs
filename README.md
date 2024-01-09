# dtrace-syscall-sensor

## Description

A DTrace-based consumer to get information about system calls on windows.

## Getting Started

Make sure you have the prerequisites for [libdtrace-rs](https://github.com/cyberphantom52/libdtrace-rs) setup.

You need to place [`dtrace.dll`](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/dtrace) file inside the executable directory manually.

### Executing program

Open PowerShell or your favorite terminal in privileged mode, clone the repo and execute 

```shell
$ cargo run
```
