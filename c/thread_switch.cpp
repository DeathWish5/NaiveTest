#include <iostream>
#include <pthread.h>
#include <unistd.h>
#include <cassert>
#include "gettime.h"

using namespace std;

const int TIMES = 10000000;

const int SCHED_POLICY[] = {SCHED_OTHER, SCHED_FIFO, SCHED_RR, SCHED_BATCH, SCHED_ISO, SCHED_IDLE, SCHED_DEADLINE};
const char *SCHED_NAME[] = {"SCHED_OTHER", "SCHED_FIFO", "SCHED_RR", "SCHED_BATCH", "SCHED_ISO", "SCHED_IDLE", "SCHED_DEADLINE"};
const int MAX = sizeof(SCHED_POLICY, sizeof(int));
int POLICY = -1;
cpu_set_t my_cpu_set;

void set_thread_sched(pthread_t t)
{
    if (pthread_setaffinity_np(t, sizeof(cpu_set_t), &my_cpu_set) != 0)
    {
        cout << "Set affinity failed. Run in root mode." << endl;
    }
    const struct sched_param para
    {
        .sched_priority = sched_get_priority_max(SCHED_POLICY[POLICY]),
    };
    if (pthread_setschedparam(t, SCHED_POLICY[POLICY], &para) != 0)
    {
        cout << "Set scheduler failed. Please run in root mode." << endl;
    }
}

void *switch_(void *_id)
{
    int id = *(int *)_id;
    pthread_yield();
    timespec start = gettime();
    for (int i = 0; i < TIMES; i++)
    {
        pthread_yield();
    }
    timespec end = gettime();
    cout << id << " " << TIMES << " switchs delta = " << (end - start) / TIMES << endl;
    return NULL;
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

    pthread_t c1, c2;
    int a0 = 1, a1 = 2;
    pthread_create(&c1, NULL, switch_, (void *)&a0);
    pthread_create(&c2, NULL, switch_, (void *)&a1);
    set_thread_sched(c1);
    set_thread_sched(c2);
    pthread_join(c1, NULL);
    pthread_join(c2, NULL);
    return 0;
}