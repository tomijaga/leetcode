from collections import deque, Counter

class FirstUnique:
    s = dict()
    q = deque()
    
    def __init__(self, nums: List[int]):
        self.s.clear()
        self.q.clear()
        
        for n in nums:
            if n in self.s:
                self.s[n] += 1
            else: 
                self.q.append(n)
                self.s[n] = 1
                
                
    def showFirstUnique(self) -> int:
        while len(self.q) > 0 and self.q[0] in self.s and self.s[self.q[0]] > 1:
            
            self.q.popleft()
            
        if len(self.q) == 0:
            return -1
        
        return self.q[0]
        
    def add(self, n: int) -> None:
        if n not in self.s:
            self.q.append(n)
            self.s[n]=1
        else:
            self.s[n]+=1