#include <assert.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
typedef struct {
	int num;
	pthread_mutex_t mutex;
} ThreadData;

void *counter(void *arg)
{
	ThreadData *data = arg;
	pthread_mutex_lock(&data->mutex);
	for (int i = 0; i < 100; i++)
		data->num++;
	pthread_mutex_unlock(&data->mutex);
	sleep(1);
	return NULL;
}

int main()
{
	ThreadData data = {
		0,
		PTHREAD_MUTEX_INITIALIZER,
	};

	pthread_t threads[10];

	for (int i = 0; i < 10; i++) {
		pthread_create(&threads[i], NULL, counter, &data);
	}

	for (int i = 0; i < 10; i++) {
		pthread_join(threads[i], NULL);
	}

	assert(data.num == 1000);
	printf("Num: %d\n", data.num);
}
