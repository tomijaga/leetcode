class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        (m, n) = (len(matrix), len(matrix[0]))
        (row, col) = (0, n-1)
        
        while row < m and col >= 0:
            num = matrix[row][col]
            
            if num > target:
                col -=1
            elif num < target:
                row +=1
            else:
                return True
        
#         if matrix[row][col] == target:
#             print('hmm')
#             print((row, col))
#             return True
        
        return False
            
                