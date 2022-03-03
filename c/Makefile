CC := g++
CPP_FLAGS := -static -O3
P := 1
PERF := 

ifeq (P, 1)
	PERF := sudo perf stat -e dTLB-load-misses,iTLB-load-misses,cs,cache-misses,sched:sched_switch
endif

%.o : %.cpp
	$(CC) $(CPP_FLAGS) -c $< -o $@

gettime: gettime.o
	$(CC) $(CPP_FLAGS) $^ -o $@

proc_switch: proc_switch.o gettime.o 
	$(CC) $(CPP_FLAGS) $^ -o $@

thread_switch: thread_switch.o gettime.o 
	$(CC) $(CPP_FLAGS) $^ -o $@ -lpthread 

clean: 
	rm *.o proc_switch thread_switch

run: 
	$(perf) sudo ./proc_switch