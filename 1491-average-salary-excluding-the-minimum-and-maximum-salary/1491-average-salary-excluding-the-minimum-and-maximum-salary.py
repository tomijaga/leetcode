import sys

class Solution:
    def average(self, salary: List[int]) -> float:
        min = sys.maxsize
        max = 0
        sum = 0
        
        for n in salary:
            if n < min:
                min = n
            if n > max:
                max = n
                
            sum +=n
        return (sum - min - max) /( len(salary) - 2)