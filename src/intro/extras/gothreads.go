package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup

	for range 2 {
		wg.Go(func() {
			fmt.Println("Hello from another thread")
		})
	}
	fmt.Println("Hello from main thread")
	wg.Wait()
}
