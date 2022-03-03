#include <iostream>
#include <iomanip>
#include "gettime.h"

using namespace std;

timespec gettime()
{
    timespec t;
    clock_gettime(CLOCK_ID, &t);
    return t;
}

ostream &operator<<(ostream &out, const timespec &t)
{
    out << t.tv_sec << "." << setw(9) << setfill('0') << t.tv_nsec;
    return out;
}

bool operator>(timespec a, timespec b)
{
    return a.tv_sec == b.tv_sec ? a.tv_nsec > b.tv_nsec : a.tv_sec > b.tv_sec;
}

bool operator<(timespec a, timespec b)
{
    return a.tv_sec == b.tv_sec ? a.tv_nsec < b.tv_nsec : a.tv_sec < b.tv_sec;
}

bool operator>=(timespec a, timespec b)
{
    return a.tv_sec == b.tv_sec ? a.tv_nsec >= b.tv_nsec : a.tv_sec > b.tv_sec;
}

bool operator<=(timespec a, timespec b)
{
    return a.tv_sec == b.tv_sec ? a.tv_nsec <= b.tv_nsec : a.tv_sec > b.tv_sec;
}

bool operator==(timespec a, timespec b)
{
    return a.tv_sec == b.tv_sec && a.tv_nsec == b.tv_nsec;
}

timespec operator/(timespec a, long b)
{
    return timespec{.tv_sec = a.tv_sec / b, .tv_nsec = ((a.tv_sec % b) * 1000000000 + a.tv_nsec) / b};
}

timespec operator-(timespec a, timespec b)
{
    timespec t{.tv_sec = a.tv_sec - b.tv_sec, .tv_nsec = a.tv_nsec - b.tv_nsec};
    if (t.tv_nsec < 0)
    {
        --t.tv_sec;
        t.tv_nsec += 1000000000L;
    }
    return t;
}

timespec operator+(timespec a, timespec b)
{
    timespec t{.tv_sec = a.tv_sec + b.tv_sec, .tv_nsec = a.tv_nsec + b.tv_nsec};
    if (t.tv_nsec >= 1000000000L)
    {
        ++t.tv_sec;
        t.tv_nsec -= 1000000000L;
    }
    return t;
}

uint64 diff_cycle(timespec start, timespec end)
{
    uint64 delta = end.tv_nsec < start.tv_nsec ? 1000000000 + end.tv_nsec - start.tv_nsec : end.tv_nsec - start.tv_nsec;
    return delta * CPU_FREQ / 1000000000;
}

int time_test()
{
    timespec time1, time2;
    time1 = gettime();
    time2 = gettime();
    cout << diff_cycle(time1, time2) << endl;
    return 0;
}
