package main

import "fmt"

func main() {
	fmt.Println(maxSubArray([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}))
}

func maxSubArray(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	max := nums[0]
	temp := 0
	for _, v := range nums {
		temp += v
		if temp > max {
			max = temp
		}
		if temp < 0 {
			temp = 0
		}
	}
	return max
}
