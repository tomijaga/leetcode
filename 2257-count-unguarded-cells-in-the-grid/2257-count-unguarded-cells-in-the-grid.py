# Each number in the grid will represent an object from 
# the problem statement
# 
# 0  -> Unguarded Cells
# 1  -> Guarded Cells
# 7  -> Guards
# 10 -> Walls

(UNGUARDED, GUARDED, GUARD, WALL) = (0, 1, 7, 10)

class Solution:
    def countUnguarded(self, m: int, n: int, guards: List[List[int]], walls: List[List[int]]) -> int:
        grid = [[0 for _ in range(0, n)] for _ in range(0, m)]
        
        cnt = ( m * n) - len(walls) - len(guards)
        
        for (row, col) in walls:
            grid[row][col] = WALL
            
        for (row, col) in guards:
            grid[row][col] = GUARD
        
        dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        
        for (row, col) in guards:
            
            for dx, dy in dirs:
                x = row + dx
                y = col + dy
                
                while ( 
                    x>=0 and 
                    x <m and 
                    y >= 0 and 
                    y< n and 
                    grid[x][y] != WALL and 
                    grid[x][y] != GUARD
                ):
                    if grid[x][y] == UNGUARDED:
                        cnt-=1
                    grid[x][y] = GUARDED
                    x = x + dx
                    y = y + dy

        return cnt  
            
        