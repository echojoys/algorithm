//插入排序,对于少量元素的排序比较有效

fn main() {

}

fn insert_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for j in 1..nums.len() {
        let key = nums[j];
        let mut i = j - 1;
        while nums[i] > key && i > 0{
            nums[i+1] = nums[i];
            i -= 1;
        }
        nums[i+1] = key;
        if nums[i] > key && i == 0{
            nums[i+1] = nums[i];
            nums[0] = key;
        }
        
    }
    nums
}



#[cfg(test)]
mod tests {
    use crate::insert_sort;
    #[test]
    fn it_works() {
        let nums = vec![5,2,4,6,1,3];
        assert_eq!(vec![1,2,3,4,5,6], insert_sort(nums));
    }
} 
