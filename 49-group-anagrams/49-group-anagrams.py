class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        map: Dict[List[int], List[str]] = {}
        
        for s in strs:
            key = [0] * 26
            
            for c in s:
                key[ord(c) - ord('a')] += 1
            
            key = tuple(key)
            
            if key in map:
                map[key].append(s)
            else:
                map[key] = [s]
            
        return map.values()