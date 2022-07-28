class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        
        if not len(s)  == len(t):
            return False
        
        map = [0] * 26
        
        for (a, b) in zip(s, t):
            if a == b:
                continue
                
            map[ord(a) - ord('a')] +=1;
            map[ord(b) - ord('a')] -=1;
            
        return all([n == 0 for n in map])