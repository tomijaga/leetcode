impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        quick_sort(nums, 0, nums.len() -1);
    }
}

pub fn quick_sort(nums: &mut Vec<i32>, start:usize, end: usize) {
    let pivot = nums[start];

    swap(nums, start, end);
    
    let mut partition = start;
    
    for i in start..end{
        let n = nums[i];
        
        if n < pivot{
            swap(nums, partition, i);
            partition +=1;
        }
    }
    
    swap(nums, partition, end);
    
    if partition > start{
        quick_sort(nums, start, partition - 1);
    }
    
    if end > partition{
        quick_sort(nums, partition + 1, end);
    }
    
}

pub fn swap(nums: &mut Vec<i32>, i: usize, j:usize){
    let mut tmp = nums[i];
    nums[i] = nums[j];
    nums[j] = tmp;
}