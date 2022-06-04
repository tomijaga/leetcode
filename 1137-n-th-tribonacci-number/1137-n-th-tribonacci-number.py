class Solution(object):
    def tribonacci(self, n):
        tri = [0, 1, 1]
        
        for i in range(3, n + 1):
            tri.append(tri[i - 3] + tri[i - 2] + tri[i - 1])
            
        return tri[n]