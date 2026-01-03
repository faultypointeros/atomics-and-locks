#include <stdio.h>
#include <pthread.h>

typedef struct {
	int *data;
	int count;
	int sum;
} ThreadData;

void *calculate_sum(void *arg)
{
	ThreadData *data = arg;
	data->sum = 0;
	for (int i = 0; i < data->count; i++) {
		data->sum += data->data[i];
	}
	return NULL;
}

int main()
{
	int arr[] = { 1, 2, 3, 4, 5 };
	ThreadData d;
	d.data = arr;
	d.count = 5;
	pthread_t p;
	if (pthread_create(&p, NULL, calculate_sum, (void *)&d) != 0) {
		perror("Failed to create thread");
		return 1;
	}
	if (pthread_join(p, NULL) != 0) {
		perror("Failed to join thread");
		return 1;
	}
	printf("The sum is: %d\n", d.sum);
}
