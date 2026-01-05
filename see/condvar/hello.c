#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

typedef struct {
	int *data;
	int len;
	int cap;
} Queue;

Queue *queue_create(int cap)
{
	Queue *q = malloc(sizeof(*q));
	q->data = malloc(sizeof(int) * cap);
	q->len = 0;
	q->cap = cap;
	return q;
}

void queue_destroy(Queue *q)
{
	free(q->data);
	free(q);
}

void queue_push(Queue *q, int num)
{
	assert(q->len < q->cap && "OUT OF MEMORY");
	q->data[q->len++] = num;
}

int queue_pop(Queue *q)
{
	if (q->len < 0)
		return -1;
	return q->data[--q->len];
}

typedef struct {
	Queue *queue;
	pthread_mutex_t *queue_mutex;
	pthread_cond_t *non_empty;
} ThreadData;

void *consumer(void *arg)
{
	ThreadData *data = arg;
	for (;;) {
		pthread_mutex_lock(data->queue_mutex);

		while (data->queue->len == 0) {
			pthread_cond_wait(data->non_empty, data->queue_mutex);
		}

		int num = queue_pop(data->queue);

		pthread_mutex_unlock(data->queue_mutex);

		printf("Num: %d\n", num);
	}
}

int main()
{
	pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
	pthread_cond_t condvar = PTHREAD_COND_INITIALIZER;
	ThreadData data;
	data.queue = queue_create(10000000);
	data.queue_mutex = &mutex;
	data.non_empty = &condvar;

	pthread_t t;
	pthread_create(&t, NULL, consumer, &data);
	for (int i = 0;; i++) {
		pthread_mutex_lock(&mutex);
		queue_push(data.queue, i);
		pthread_mutex_unlock(&mutex);
		pthread_cond_signal(&condvar);
		sleep(1);
	}
	pthread_join(t, NULL);
	return 0;
}
