fn main() {
    mmc_1_to_10();

}


fn mmc_1_to_10 ()   {
    let mut nums: [u8; 20] = [0; 20];
    let mut current_divisor = 2;
    for i in 0..20 {
        nums[i] += 1 + i as u8;
    }

    for i in 0..20 {
       if nums[i] % current_divisor == 0 {
            nums[i] = nums[i] / current_divisor;
       } 
    }

    for i in nums {
        println!("{}", i);
    }
}
