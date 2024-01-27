package main

import (
	"fmt"
	"sort"
)

func main() {
	fmt.Println(fourSum1([]int{1, 0, -1, 0, -2, 2}, 0))
}

func fourSum1(nums []int, target int) [][]int {
	var res [][]int
	var indices [][]int
	oMap := make(map[int]int)
	for i := 0; i < len(nums)-2; i++ {
		for j := i + 1; j < len(nums)-1; j++ {
			for k := j + 1; k < len(nums); k++ {
				foo := target - (nums[i] + nums[j] + nums[k])
				index, contains := oMap[foo]
				if contains && !contains1(indices, []int{i, j, k, index}) {
					res = insert(res, []int{nums[i], nums[j], nums[k], foo})
					indices = append(indices, []int{i, j, k, index})
				} else {
					oMap[nums[k]] = k
				}
			}
		}
	}
	return res
}

func contains1(arr1 [][]int, arr2 []int) bool {
	for _, v := range arr1 {
		if arrEq(v, arr2) {
			return true
		}
	}
	return false
}

func fourSum(nums []int, target int) [][]int {
	var res [][]int
	sort.Ints(nums)
	for i := 0; i < len(nums)-3; i++ {
		for j := i + 1; j < len(nums)-2; j++ {
			for k := j + 1; k < len(nums)-1; k++ {
				for l := k + 1; l < len(nums); l++ {
					if nums[i]+nums[j]+nums[k]+nums[l] == target {
						res = insert(res, []int{nums[i], nums[j], nums[k], nums[l]})
					}
				}
			}
		}
	}
	return res
}

func insert(arr1 [][]int, arr2 []int) [][]int {
	for _, v := range arr1 {
		if arrEq(v, arr2) {
			return arr1
		}
	}
	newarr := append(arr1, arr2)
	return newarr
}

func arrEq(arr1 []int, arr2 []int) bool {
	if len(arr1) != len(arr2) {
		return false
	}
	for i := 0; i < len(arr1); i++ {
		if arr1[i] != arr2[i] {
			return false
		}
	}
	return true
}
