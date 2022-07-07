class Solution:
    def countLatticePoints(self, circles: List[List[int]]) -> int:
        s= set()
        
        for (x, y, r) in circles:
            for i in range(x - r, x + r + 1):
                for j in range(y - r, (y + r) + 1):
                    dx = abs(i - x)
                    dy = abs(j - y)
                    
                    if sqrt(pow(dx, 2) + pow(dy, 2)) <= r:
                        s.add((i, j))
        # print(s)
        return len(s)