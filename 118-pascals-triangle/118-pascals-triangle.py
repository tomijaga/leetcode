from itertools import pairwise

class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        pascal = [[1]]
        
        row = [1]
        for i in range(1, numRows):
            for (a, b) in pairwise(pascal[-1]):
                
                row.append(a + b)
            
            row.append(1)
            pascal.append(row)
            row = [1]
        
        return pascal