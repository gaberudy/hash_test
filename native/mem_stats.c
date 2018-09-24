 #include <unistd.h>
 #include <stdio.h>
 #include "runlib.h"
static RunProcDyn rpd0;

void mem_start()
{
	pid_t pid = getpid();
	run_get_dynamic_proc_info(pid, &rpd0);
}

void mem_stop()
{
    RunProcDyn rpd1;
	double ut, st;
	pid_t pid = getpid();
	run_get_dynamic_proc_info(pid, &rpd1);
	ut = rpd1.utime - rpd0.utime;
	st = rpd1.stime - rpd0.stime;
	printf("CPU time: %.3lf (= %.3lf + %.3lf)\n", ut + st, ut, st);
    printf("RSS mem: %.3lf kB\n", (rpd1.rss - rpd0.rss) / 1024.0);
}