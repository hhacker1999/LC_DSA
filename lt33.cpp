#include <iostream>
#include <vector>
using namespace std;

int search(vector<int> &nums, int target) {

  int start = 0;
  int end = nums.size() - 1;
  int pivot = 0;
  while (start < end) {
    int mid = (start + end) / 2;
    if (nums[mid] > nums[mid + 1]) {
      pivot = mid;
    } else if (nums[mid - 1] > nums[mid]) {
      pivot = mid;

    } else if (nums[start] > nums[mid]) {
      start = mid + 1;
    } else {
      end = mid - 1;
    }
  }

  if (nums[pivot] == target) {
    return pivot;
  }
  if (nums[0] < target) {
    int end2 = pivot;
    int start2 = 0;
    while (start2 < end2) {
      int mid = (start2 + end2) / 2;
      if (nums[mid] == target) {
        return mid;
      } else if (nums[mid] > target) {
        end2 = mid - 1;
      } else {
        start2 = mid + 1;
      }
    }
  } else {
    int start3 = pivot;
    int end3 = nums.size() - 1;
    while (start3 < end) {
      int mid = (start3 + end) / 2;
      if (nums[mid] == target) {
        return mid;
      } else if (nums[mid] > target) {
        end3 = mid - 1;
      } else {
        start3 = mid + 1;
      }
    }
  }

  // while (start < end) {
  //   int mid = (start + end) / 2;
  //   int currEle = nums[mid];
  //   int nextEle = nums[mid + 1];
  //   if (currEle == target) {
  //     return mid;
  //   } else if (currEle < target && nextEle > currEle) {
  //     start = mid + 1;
  //   } else {
  //     end = mid - 1;
  //   }
  // }
  return -1;
}
