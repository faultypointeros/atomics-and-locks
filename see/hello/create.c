#include <pthread.h>
#include <stdio.h>

void *f(void *arg)
{
	(void)arg;
	pthread_t self = pthread_self();
	printf("Hello from thread %lu\n", self);
	return NULL;
}

int main()
{
	pthread_t p1, p2;
	pthread_create(&p1, NULL, f, NULL);
	pthread_create(&p2, NULL, f, NULL);
	printf("Hello from main thread\n");
	pthread_join(p1, NULL);
	pthread_join(p2, NULL);
	return 0;
}
