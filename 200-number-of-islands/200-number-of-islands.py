from collections import deque

class Solution:
    
    def numIslands(self, grid: List[List[str]]) -> int:
        num = 0
        
        for i in range(0, len(grid)):
            for j in range(0, len(grid[0])):
                if grid[i][j] == '1':
                    num += self.bfs(grid, (i, j))
                    # print("------")
        return num
                
    def bfs(self, grid: List[List[str]], point: (int, int)) -> bool:
        (m, n) = (len(grid), len(grid[0]))
        (i, j) = point
        
        if i < 0 or j < 0 or i >= m or j >= n:
            return False
        
        if grid[i][j] != '1':
            return False
        
        # print(True, (i, j))
        
        # mark as visited
        grid[i][j] = '#'
        
        self.bfs(grid, (i - 1, j))
        self.bfs(grid, (i + 1, j))
        self.bfs(grid, (i, j - 1))
        self.bfs(grid, (i, j + 1))

        return True