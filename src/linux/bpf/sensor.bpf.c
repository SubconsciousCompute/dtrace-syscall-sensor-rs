#include "vmlinux.h"
#include <linux/version.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include "sensor.h"

char LICENSE[] SEC("license") = "GPL";

struct exec_event e = {0};

struct
{
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} exec_rb SEC(".maps");

struct exec_args_t
{
    __u64 __unused;
    __u64 __unused2;
    char *filename;
};

SEC("tp/syscalls/sys_enter_execve")
int syscall_execve_probe(struct trace_event_raw_sys_enter *ctx)
{
    struct task_struct *task;
    pid_t pid, tid;
    u64 id, ts, *start_ts, start_time = 0;
    struct exec_event *event;

    id = bpf_get_current_pid_tgid();
    pid = id >> 32;
    tid = (u32) id;

    if (pid != tid) return 0;
    
    event = bpf_ringbuf_reserve(&exec_rb, sizeof(struct exec_event), 0);
    if (!event)
        return 0;
    
    task = (struct task_struct *)bpf_get_current_task();
    start_time = BPF_CORE_READ(task, start_time);
    event->pid = pid;
    event->exit_code = (BPF_CORE_READ(task, exit_code) >> 8) & 0xff;
    event->ppid = BPF_CORE_READ(task, real_parent, tgid);
    bpf_get_current_comm(&event->comm, sizeof(event->comm));
    // store the full filename
    bpf_probe_read_str(&event->filename, sizeof(event->filename), (void *)ctx->args[0]);
    
    bpf_ringbuf_submit(event, 0);
    return 0;
}

/*
struct open_args_t
{
    char *filename;
    int flags;
    int mode;
};

SEC("tp/syscalls/sys_enter_open")
int syscall_open_probe(struct trace_event_raw_sys_enter *ctx)
{
    u64 id = bpf_get_current_pid_tgid();
    u32 pid = id >> 32;
    u32 tid = id;

    struct open_args_t args = {};
    args.filename = (const char *)ctx->args[0];
    args.flags = (int)ctx->args[1];
    return 0;
}

SEC("tp/syscalls/sys_enter_openat")
int syscall_openat_probe(struct trace_event_raw_sys_enter *ctx)
{
    u64 id = bpf_get_current_pid_tgid();
    u32 pid = id >> 32;
    u32 tid = id;

    struct open_args_t args = {};
    args.filename = (const char *)ctx->args[1];
    args.flags = (int)ctx->args[2];
    return 0;
}

SEC("tp/syscalls/sys_enter_fork")
int syscall_fork_probe(struct trace_event_raw_sys_enter *ctx)
{
    return 0;
}


SEC("tp/syscalls/sys_enter_execveat")
int syscall_execveat_probe(struct trace_event_raw_sys_enter *ctx)
{
    return 0;
}
*/

