class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        
        left = 0
        right = len(numbers) - 1
        
        while True:
     
            sum = numbers[left] + numbers[right]
            
            if sum > target:
                right-=1
            elif sum < target:
                left +=1
            else:
                return [left + 1, right + 1]