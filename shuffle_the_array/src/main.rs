fn main() {
    println!("Hello, world!");
    let nums = vec![2,5,1,3,4,7]; 
    let n = 3;
    let results = shuffle(nums, n);
    println!("results: {:?}", results);
}


fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();    
    let (right, left) = nums.split_at(n as usize);
    println!("right {:?} left: {:?}", right, left);
    for i in 0..n as usize {
        results.push(right[i]);
        results.push(left[i]);
    } 
    println!("results vec: {:?}", results);
    results
}