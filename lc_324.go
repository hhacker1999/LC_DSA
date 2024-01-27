// TODO: current implementation does not cover all of the edge cases
package main

import "fmt"

func main() {
	nums := []int{5, 5, 5, 4, 4, 4, 4}
	fmt.Println(nums)
	wiggleSort(nums)
	fmt.Println(nums)

}

func wiggleSort(nums []int) {
	for !isWiggleSorted(nums) {
		for i := 0; i < len(nums)-2; i++ {
			if nums[i+1] == nums[i] {
				continue
			}
			if i%2 == 0 {
				if nums[i+2] <= nums[i] {
					temp := nums[i]
					nums[i] = nums[i+2]
					nums[i+2] = temp
				}
			} else {
				if nums[i+2] >= nums[i] {
					temp := nums[i]
					nums[i] = nums[i+2]
					nums[i+2] = temp
				}
			}
		}
	}
}

func isWiggleSorted(nums []int) bool {
	for i := 0; i < len(nums)-1; i++ {
		if i%2 == 0 {
			if nums[i+1] <= nums[i] {
				return false
			}
		} else {
			if nums[i+1] >= nums[i] {
				return false
			}
		}
	}
	return true
}
