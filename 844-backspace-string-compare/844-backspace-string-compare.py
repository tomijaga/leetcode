class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        a = []
        b = []
        
        skip = 0
        for c in s[::-1]:
            if c == '#':
                skip +=1
            elif skip > 0:
                skip -=1
            else:
                a.append(c)
                
        skip = 0
                
        for c in t[::-1]:
            if c == '#':
                skip +=1
            elif skip > 0:
                skip -=1
            else:
                b.append(c)
        
        return a == b
                
            
            
                