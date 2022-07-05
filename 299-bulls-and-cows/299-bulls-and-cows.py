class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        s = set(secret)
        m : dict[str, int]= {}
        
        for c in secret:
            if c in m:
                m[c] +=1
            else:
                m[c] = 1
                
        bulls = 0
        cows = 0
        
        no_match =[]
        for i in range(0, len(secret)):
            if secret[i] == guess[i]:
                m[guess[i]]-=1
                bulls +=1
            else:
                no_match.append(i)
                
        for i in no_match:
            if guess[i] in m and m[guess[i]] > 0:
                
                m[guess[i]]-=1
                cows+=1
                
        return str(bulls) + "A" + str(cows) + "B"