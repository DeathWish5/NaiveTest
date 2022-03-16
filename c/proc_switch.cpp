#include <iostream>
#include <unistd.h>
#include <sched.h>
#include <wait.h>
#include <cassert>
#include "gettime.h"

using namespace std;

const int SCHED_POLICY[] = {SCHED_OTHER, SCHED_FIFO, SCHED_RR, SCHED_BATCH, SCHED_ISO, SCHED_IDLE, SCHED_DEADLINE};
const char *SCHED_NAME[] = {"SCHED_OTHER", "SCHED_FIFO", "SCHED_RR", "SCHED_BATCH", "SCHED_ISO", "SCHED_IDLE", "SCHED_DEADLINE"};
const int MAX = sizeof(SCHED_POLICY, sizeof(int));
int POLICY = -1;

cpu_set_t my_cpu_set;

void set_sched()
{
    if (sched_setaffinity(0, sizeof(cpu_set_t), &my_cpu_set) != 0)
    {
        cout << "Set affinity failed. Run in root mode." << endl;
    }
    const struct sched_param para
    {
        .sched_priority = sched_get_priority_max(SCHED_POLICY[POLICY]),
    };
    if (sched_setscheduler(0, SCHED_POLICY[POLICY], &para) != 0)
    {
        cout << "Set scheduler failed. Please run in root mode." << endl;
    }
}

const int TIMES = 10000000;

void switch_test(const char name[])
{
    sched_yield();
    timespec start = gettime();
    for (int i = 0; i < TIMES; i++)
    {
        sched_yield();
    }
    timespec end = gettime();
    cout << name << " " << TIMES << " switchs delta = " << (end - start) / TIMES << endl;
}

int get_priority()
{
    struct sched_param para;
    sched_getparam(0, &para);
    return para.sched_priority;
}

int main(int argc, char *argv[])
{
    POLICY = (argc == 2) ? atoi(argv[1]) : 1;
    assert(POLICY >= 0 && POLICY < MAX);
    cout << "Sched Policy: " << SCHED_NAME[POLICY] << endl;
    CPU_ZERO(&my_cpu_set);
    const unsigned int cpu = sched_getcpu();
    cout << "Run in cpu #" << cpu << endl;
    CPU_SET(cpu, &my_cpu_set);
    set_sched();
    if (fork() == 0)
    {
        set_sched();
        switch_test("C");
        exit(0);
    }
    switch_test("F");
    wait(NULL);
    return 0;
}