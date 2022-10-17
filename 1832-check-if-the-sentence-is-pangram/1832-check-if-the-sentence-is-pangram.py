class Solution:
    def checkIfPangram(self, sentence: str) -> bool:
        letters = set(sentence)
        
        return len(letters) == 26