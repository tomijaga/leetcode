import heapq

class MedianFinder:
    def __init__(self):
        self.min_heap = []
        self.max_heap = []
        self.len = 0

    def addNum(self, num: int) -> None:
        
        if self.len == 0:
            heapq.heappush(self.max_heap, -num)
            self.len+=1
            return 
        
        if num > -self.max_heap[0]:
            heapq.heappush(self.min_heap, num)
        else:
            heapq.heappush(self.max_heap, -num)
        self.len+=1
        
        # print(self.max_heap, self.min_heap)
        
        mid = floor(self.len / 2)
        
        if len(self.max_heap) > mid:
            num = heapq.heappop(self.max_heap) * -1
            heapq.heappush(self.min_heap, num)
            
        if len(self.min_heap) > len(self.max_heap):
            num = heapq.heappop(self.min_heap)
            heapq.heappush(self.max_heap, -num)
        
        
    def findMedian(self) -> float:
        
        if self.len % 2 == 0:
            return (-self.max_heap[0] + self.min_heap[0])/2
        else:
            return -self.max_heap[0]
