#include <iostream>
#include <vector>

using namespace std;

int main() { return 0; }

// [4,5,6,7,0,1,2]
int search(vector<int> &nums, int target) {
  int start = 0;
  int end = nums.size() - 1;
  while (start <= end) {
    int mid = (start + end) / 2;
    int value = nums[mid];
    if (value < target) {
      start = mid + 1;
    } else if (value == target) {
      return mid;
    } else {
      if (nums[0] < target) {
        end = mid - 1;
      } else {
        start = mid + 1;
      }
    }
  }
  return -1;
}
