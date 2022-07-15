class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> bool:
        def dfs(i: int, j: int)-> int:
            sum = 0
            if i >= 0 and j >= 0 and i < m and j < n:
                if grid[i][j] == 1:
                    grid[i][j] = 0
                    sum = 1
                    for (dx, dy) in [(1 ,0), (0, 1), (-1, 0), (0, -1)]:
                        x = i + dx
                        y = j + dy
                        sum+=dfs(x, y)

            if sum >=1:
                print(i, j)
            return sum
                    
        m, n = len(grid), len(grid[0])
        cnt = 0
        for i in range(0, m):
            for j in range(0, n):
                if grid[i][j] == 1:
                    cnt = max(dfs(i, j), cnt)
                    print("---------")
                    
        return cnt