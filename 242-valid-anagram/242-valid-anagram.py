class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if not (len(s) ==  len(t)):
            return False
        
        _t = len(t)
        
        for c in s:
            if c in t:
                i = t.index(c)
                t = t[0:i] + t[i+1:_t]
            else: 
                return False
        return True