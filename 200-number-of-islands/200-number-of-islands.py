from collections import deque

class Solution:
    
    def numIslands(self, grid: List[List[str]]) -> int:
        visited = set()
        q = deque()
        num = 0
        
        for i in range(0, len(grid)):
            for j in range(0, len(grid[0])):
                num += self.dfs(grid, visited, q, (i, j))
                # print("------")
        return num
                
    def dfs(self, grid: List[List[str]], visited, q, point: (int, int)) -> bool:
        (m, n) = (len(grid), len(grid[0]))
        (i, j) = point
        
        if i < 0 or j < 0 or i >= m or j >= n:
            return False
        
        if point in visited or grid[i][j] == '0':
            return False
        
        # print(True, (i, j))
        visited.add(point)
        
        q.append((i - 1, j))
        q.append((i + 1, j))
        q.append((i, j - 1))
        q.append((i, j + 1))
        
        while len(q) > 0:
            self.dfs(grid, visited, q, q.popleft())
        
        return True