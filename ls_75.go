package main

import "fmt"

func main() {
	nums := []int{2, 0, 2, 1, 1, 0}
	fmt.Println(nums)
	sortColors(nums)
	fmt.Println(nums)
}

func sortColors(nums []int) {
	for !isSorted(nums) {
		for i := 0; i < len(nums) - 1; i++ {
			if nums[i] > nums[i+1] {
				temp := nums[i]
				nums[i] = nums[i+1]
				nums[i+1] = temp
			}
		}
	}
}

func isSorted(nums []int) bool {
	for i := 0; i < len(nums)-1; i++ {
		if nums[i] > nums[i+1] {
			return false
		}
	}
	return true
}
