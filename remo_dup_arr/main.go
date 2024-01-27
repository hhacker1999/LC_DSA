package main

import "fmt"

// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
//
// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.

// {0, 1, 1, 1, 1, 2, 2, 3, 3, 4}
// {0, 1, 2, 2, 4}
// {1, 2, 3}

func main() {
	fmt.Println(removeDuplicate([]int{1, 2, 3}))

}

// Main hint was that we want to put the next not duplicate in front of last not duplicate this way
// we get an array with all prefix values unique. To reach here I first just tried to get the unique
// elements length and break it into smaller problem
func removeDuplicate(nums []int) int {
	first := 0
	second := 1
	for {
		if second >= len(nums) {
			break
		}
		if nums[first] != nums[second] {
			nums[first+1] = nums[second]
			first += 1
			second += 1
		} else {
			second += 1
		}
	}
	return first + 1
}
