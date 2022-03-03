#pragma once

#include <iostream>
#include <time.h>
#include <sys/resource.h>

typedef unsigned long long uint64;

// #define CLOCK_ID (CLOCK_REALTIME)
#define CLOCK_ID (CLOCK_MONOTONIC)
// #define CLOCK_ID (CLOCK_PROCESS_CPUTIME_ID)

#define CPU_FREQ (1024 * 1024 * 1024)

timespec gettime();

std::ostream &operator<<(std::ostream &out, const timespec &t);
bool operator>(timespec a, timespec b);
bool operator<(timespec a, timespec b);
bool operator>=(timespec a, timespec b);
bool operator<=(timespec a, timespec b);
bool operator==(timespec a, timespec b);
timespec operator-(timespec a, timespec b);
timespec operator+(timespec a, timespec b);
timespec operator/(timespec a, long b);

uint64 diff_cycle(timespec start, timespec end);