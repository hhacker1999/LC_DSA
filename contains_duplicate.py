from typing import List

class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        list1 = set()
        contains = False
        for i in nums:
            if i in list1:
                contains = True
                break
            list1.add(i)
        return contains