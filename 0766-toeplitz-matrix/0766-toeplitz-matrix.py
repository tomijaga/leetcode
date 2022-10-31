class Solution:
    def isToeplitzMatrix(self, matrix: List[List[int]]) -> bool:
        (rows, cols) = (len(matrix), len(matrix[0]))
        
        def isDiagonal(i, j):
            n = matrix[i][j]
            
            while 0 <= i < rows and 0 <= j < cols:
                if matrix[i][j] != n:
                    return False
                
                i+= 1
                j+= 1
            
            return True
                
        for i in range(0, rows):
            if not isDiagonal(i, 0):
                return False
            
        for j in range(0, cols):
            if not isDiagonal(0, j):
                return False
            
        return True
            
           