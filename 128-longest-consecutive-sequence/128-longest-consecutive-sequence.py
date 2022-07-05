class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        s = set(nums)
        
        max_cnt = 0
        
        for n in s:
            if (n - 1) not in s:
                curr = n
                cnt = 0
                while curr in s:
                    cnt += 1
                    curr +=1
                    max_cnt = max(max_cnt, cnt)
            
        return max_cnt