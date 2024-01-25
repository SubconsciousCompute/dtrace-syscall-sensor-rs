#ifndef __SENSOR_H
#define __SENSOR_H

#define TASK_COMM_LEN 16
struct exec_event
{
    int pid;
    int ppid;
    int exit_code;
    char comm[TASK_COMM_LEN];
    char filename[512];
};

#endif