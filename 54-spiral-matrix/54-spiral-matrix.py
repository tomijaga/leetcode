class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        m, n = len(matrix), len(matrix[0])
        
        grid = [] 
        top, bottom, left, right = 0, m, 0, n
        
        filledGridLen = m*n
        
        while len(grid) < filledGridLen:
            for j in range(left, right):
                grid.append(matrix[top][j])
                
            top += 1
            
            if len(grid) < filledGridLen:
                for i in range(top, bottom):
                    grid.append(matrix[i][right - 1])
            
            right -= 1
            
            if len(grid) < filledGridLen:
                for j in reversed(range(left, right)):
                    grid.append(matrix[bottom - 1][j])
                    
            bottom -= 1
            
            if len(grid) < filledGridLen:
                for i in reversed(range(top, bottom)):
                    grid.append(matrix[i][left])
            
            left +=1
        return grid