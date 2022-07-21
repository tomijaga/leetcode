class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = [[]]
        
        def ss(nums:List[int], arr: List[int]):
            
            for (i, n) in enumerate(nums):
                arr.append(n)
                res.append([x for x in arr])
                
                ss(nums[i + 1:], arr)
                arr.pop()
                
        ss(nums, [])
        
        return res