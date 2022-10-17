class Solution:
    def checkIfPangram(self, sentence: str) -> bool:
        letters = [False] * 26
        cnt = 0
        
        for c in sentence:
            if letters[ord(c) - ord("a")] == False:
                cnt += 1
                letters[ord(c) - ord("a")] = True
            
            if cnt == 26:
                return True
        
        return False