from collections import deque

class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        rotten = deque()
        fresh = 0
        (m, n) = (len(grid), len(grid[0]))
        
        for i in range(0, m):
            for j in range(0, n):
                if grid[i][j] == 2:
                    rotten.append((i, j))
                elif grid[i][j] == 1:
                    fresh+=1
                
        dirs = [(1, 0),(0, 1), (-1, 0), (0, -1)]
        days = 0
        while len(rotten) > 0:
            changed = False
            # print(rotten)
            for _ in range(0, len(rotten)):
                (x, y) = rotten.popleft()
                
                for (dx, dy) in dirs:
                    i = x + dx
                    j = y + dy
                    
                    if i >= 0 and j >= 0 and i < m and j < n:
                        
                        if grid[i][j] == 1:
                            grid[i][j] = 2
                            fresh-=1
                            changed = True
                            rotten.append((i, j))
                            
            if changed:
                days +=1
        
        if fresh > 0:
            return -1
        
        return days
                    
            