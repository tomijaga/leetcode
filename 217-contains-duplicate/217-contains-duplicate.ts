function containsDuplicate(nums: number[]): boolean {
    let set = new Set();
    
    for (const n of nums){
        if (set.has(n)) {
            return true
        }else{
            set.add(n)
        }
    }
    
    return false
};