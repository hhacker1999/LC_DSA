#include <cstdlib>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

int longestConsecutive(vector<int> &nums) {
  unordered_set<int> arrSet;
  int count = 0;
  for (int i = 0; i < nums.size(); i++) {
    arrSet.insert(nums[i]);
  }
  for (auto i : arrSet) {
    if (arrSet.count(i - 1)) {
      continue;
    }
    int index = 0;
    while (arrSet.count(i + index)) {
      index += 1;
    }
    if (count < index) {
      count = index;
    }
  }
  return count;
}

int main() {}
