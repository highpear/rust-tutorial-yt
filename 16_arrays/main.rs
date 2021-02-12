fn main() {

    let nums = [1, 2, 3, 4, 5];

    for n in nums.iter() {
        println!("{}", n);
    }
    
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    let zeros = [0; 20];
    println!("{} x {}", zeros[0], zeros.len());
    
}