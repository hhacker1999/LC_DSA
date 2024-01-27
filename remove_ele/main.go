package main

import "fmt"

// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
//
// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.

func main() {
	fmt.Println(removeElement([]int{0, 1, 2, 2, 3, 0, 4, 2}, 2))

}

func removeElement(nums []int, val int) int {
	firstPtr := 0
	lastUnique := -1
	secondPtr := 1

	for {
		if secondPtr >= len(nums) {
			break
		}
		if nums[firstPtr] == val {
			if nums[secondPtr] != val {
				lastUnique += 1
				nums[lastUnique] = nums[secondPtr]
				firstPtr = secondPtr + 1
				secondPtr += 2
			} else {
				secondPtr += 1
			}
		} else {
			lastUnique += 1
			nums[lastUnique] = nums[firstPtr]
			firstPtr += 1
			secondPtr += 1
		}
	}
	return lastUnique
}
