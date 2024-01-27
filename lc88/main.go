package main

import (
	"fmt"
	"sort"
)

func main() {

	arr := []int{2, 0}
	arr1 := []int{1}
	fmt.Println(arr)
	merge1(arr, 1, arr1, 1)
	fmt.Println(arr)
}

// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

func merge(nums1 []int, m int, nums2 []int, n int) {
	j := 0
	for i := m; i < m+n; i++ {
		nums1[i] = nums2[j]
		j += 1
	}
	sort.Ints(nums1)
}

// [1,2,3,0,0,0]
// [2,5,6]
// [1, 2, 2, 3, 5, 6]
// [0]
// [1]
func merge1(nums1 []int, m int, nums2 []int, n int) {
	i := m - 1
	j := n - 1
	k := len(nums1) - 1
	for {
		if j < 0 {
			break
		}
		if i >= 0 && nums2[j] >= nums1[i] {
			nums1[k] = nums2[j]
			j -= 1
		} else {
			nums1[k] = nums1[i]
			i -= 1
		}
		k -= 1
	}
}

// [ 1, 1, 1, 1, 3, 2, 0, 0, 0]
func shift(arr []int, index int, value int) {
	length := len(arr)
	if length < index {
		return
	}

	for i := length - 1; i > index; i-- {
		arr[i] = arr[i-1]
	}

	arr[index] = value
}
