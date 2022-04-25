class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        dict = {}
        for n in nums1:
            if n in dict:
                dict[n]+=1
            else:
                dict[n] = 1
        
        result = []
        for n in nums2:
            if n in dict:
                if dict[n] > 0:
                    dict[n]-=1
                    result.append(n)
                
        return result