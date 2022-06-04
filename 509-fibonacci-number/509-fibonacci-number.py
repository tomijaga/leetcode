class Solution:
    array = [0, 1]
    
    def fib(self, n: int) -> int:
        f = 0
        if n == 0 or n ==1:
            return self.array[n]
        elif n <= len(self.array):
            f = self.array[n-1] + self.array[n-2]
            self.array.append(f)
        else:
            self.fib(n-1)
            f = self.array[n-1] + self.array[n-2]
            self.array.append(f)
        return f
        
        