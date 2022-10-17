class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        max_len = 0
        
        last_seen = dict()
        i = 0
        
        for (j, c) in enumerate(s):
            if c in last_seen:
                i = max(last_seen[c] + 1, i)
                
            last_seen[c] = j
            max_len = max(max_len, (j - i) + 1)
            
        return max_len