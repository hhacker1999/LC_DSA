#include <cstdio>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;
// [-1,0,1,2,-1,-4]
vector<vector<int>> three_sum(vector<int> &nums) {
  vector<vector<int>> ans;
  unordered_set<int> tempStorage;
  int index1 = 0;
  int index2 = nums.size() - 1;
  while (index1 < index2) {
    vector<int> maybe;
    int remaining = 0 - (nums[index1] + nums[index2]);
    if (tempStorage.count(remaining)) {
      if (tempStorage.count(remaining) && (!tempStorage.count(nums[index1]) ||
                                           !tempStorage.count(nums[index2]))) {
        maybe.push_back(nums[index1]);
        maybe.push_back(nums[index2]);
        maybe.push_back(remaining);
        ans.push_back(maybe);
      }
    } else {
      tempStorage.insert(nums[index2]);
    }
    index2 -= 1;
  }
  return ans;
}
