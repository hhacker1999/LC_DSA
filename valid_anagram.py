class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        lengthEqual: bool = len(s) == len(t)
        contains = True
        for i in s:
            if i not in t:
                contains = False
                break
        isAnagram: bool = contains and len
        return isAnagram
