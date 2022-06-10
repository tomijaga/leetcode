class Solution:
    def arrayChange(self, nums: List[int], operations: List[List[int]]) -> List[int]:
        map = {}
        
        for (i, n) in enumerate(nums):
            map[n] = i
            
        for [old, new] in (operations):
            i = map.pop(old)
            map[new] = i
            
        res = [0 for _ in range(0, len(map))]
        
        for (n, i) in map.items():
            res[i] = n
        
        return res
        