package main

import (
	"fmt"
)

// Given an array of integers nums, half of the integers in nums are odd, and the other half are even.
//
// Sort the array so that whenever nums[i] is odd, i is odd, and whenever nums[i] is even, i is even.
//
// Return any answer array that satisfies this condition.

func main() {
	fmt.Println(sortArrayByParityII([]int{4, 2, 5, 7}))
}

func sortArrayByParityII(nums []int) []int {
	odd := []int{}
	even := []int{}

	for _, val := range nums {
		if val%2 == 0 {
			even = append(even, val)
		} else {
			odd = append(odd, val)
		}
	}
	evnPtr := 0
	oddPtr := 1

	for i := 0; i < len(even); i++ {
		nums[evnPtr] = even[i]
		nums[oddPtr] = odd[i]
		evnPtr += 2
		oddPtr += 2
	}

	return nums
}
