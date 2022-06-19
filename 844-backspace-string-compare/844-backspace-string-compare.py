class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        a = []
        b = []
        
        for c in s:
            if c == '#':
                if len(a) > 0:
                    a.pop()
            else:
                a.append(c)
                
        for c in t:
            if c == '#':
                if len(b) > 0:
                    b.pop()
            else:
                b.append(c)
        
        return a == b
                
            
            
                