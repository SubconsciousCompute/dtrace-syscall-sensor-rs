#ifndef __SENSOR_H
#define __SENSOR_H

#define TASK_COMM_LEN 16
struct exec_event
{
    uint pid;
    uint ppid;
    int exit_code;
    u64 timestamp;
    char comm[TASK_COMM_LEN];
    char filename[512];
};

#endif