class Solution:
    def mySqrt(self, x: int) -> int:
        left:int = 0
        right:int = x
        
        while left < right:
            mid = int(left + (right - left)/2)
            
            if (mid **2 > x):
                right = mid -1
            elif ((mid **2) < x):
                print(mid)
                if (mid+1)**2 == x:
                    return mid +1
                elif (mid+1)**2 > x:
                    return mid
                    
                left = mid+1
            else:
                return mid
        
        # print("left")
        return left
                    