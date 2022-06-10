class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        
        rows = set()
        cols = set()
        
        for i in range(0, m):
            for j in range(0, n):
                if matrix[i][j] == 0:
                    rows.add(i)
                    cols.add(j)
                    
        for i in rows:
            for j in range(0, n):
                matrix[i][j] = 0
                
        for j in cols:
            for i in range(0, m):
                matrix[i][j] = 0