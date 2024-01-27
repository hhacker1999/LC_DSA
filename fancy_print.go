package main

import (
	"fmt"
	"time"
)

func main() {
	fancyPrint("FaNcy!! PrInT ExamPLE :)", time.Millisecond*10)

}

func fancyPrint(target string, delay time.Duration) {
	str := ""
	next := "h"
	for str != target {
		time.Sleep(delay)
		next = string(target[len(str)])
		var temp string
		if int(next[0]) >= 65 && int(next[0]) <= 90 {
			temp = "A"
		} else if int(next[0]) >= 97 && int(next[0]) <= 122 {
			temp = "a"
		} else {
			temp = next
		}
		str += temp
		for temp != next {
			fmt.Println(str)
			foo := int(temp[0])
			foo += 1
			temp = string(foo)
			str = str[:len(str)-1] + temp
			time.Sleep(delay)
		}
		fmt.Println(str)
	}
}
