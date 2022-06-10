import heapq

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        map = {}
        
        for n in nums:
            if n in map:
                map[n]+= 1
            else:
                map[n] = 1
                
        heap = []
        for (n, cnt) in map.items():
            if len(heap) < k:
                heappush(heap, (cnt, n))
            else:
                heappushpop(heap, (cnt, n))
                
        return [heappop(heap)[1] for _ in range(0, len(heap))]