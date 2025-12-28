#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
void *f(void *arg) {
  printf("Hello from another thread\n");
  return NULL;
}
int main() {
  pthread_t t1, t2;
  pthread_create(&t1, NULL, f, NULL);
  pthread_create(&t2, NULL, f, NULL);
  printf("Hello from main thread\n");
  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  return EXIT_SUCCESS;
}
