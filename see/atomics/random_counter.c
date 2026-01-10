#include <string.h>
#include <stdio.h>
#include <pthread.h>
#include <time.h>
#include <unistd.h>
#include <stdatomic.h>
#define DURATION_LENGTH 5

atomic_bool STOP;
atomic_ullong COUNTER;
void *random_counter(void *arg)
{
	(void)arg;
	int sleep_durations[DURATION_LENGTH] = { 4, 6, 2, 9, 1 };
	while (!atomic_load_explicit(&STOP, memory_order_relaxed)) {
		unsigned long long idx = atomic_fetch_add_explicit(
			&COUNTER, 1, memory_order_relaxed);
		struct timespec slp = {
			0,
			sleep_durations[idx % DURATION_LENGTH] * 1e8,
		};
		nanosleep(&slp, NULL);
	}
	return NULL;
}

int main()
{
	pthread_t t;
	pthread_create(&t, NULL, random_counter, NULL);
	char cmd[100];
	while (1) {
		(void)scanf("%s", cmd);
		if (strcmp(cmd, "help") == 0) {
			printf("commands: help, stop, print\n");
		} else if (strcmp(cmd, "stop") == 0) {
			break;
		} else if (strcmp(cmd, "print") == 0) {
			unsigned long long counter = atomic_load_explicit(
				&COUNTER, memory_order_relaxed);
			printf("counter: %lld\n", counter);
		} else {
			printf("unknown command: %s\n", cmd);
		}
	}
	return 0;
}
