const RANGE: usize = 50;

fn main() {
    let mut nums: [i32; RANGE] = [0; RANGE];
    let mut curr_mmc = 1;
    let mut divisor = 2;

    for i in 0..nums.len() {
        nums[i] = 1 + i as i32;
    }

   while !is_all_one(nums) {
        if can_divide_something(nums, divisor) {
            for i in 0..nums.len() {
                nums[i] = 
                    if nums[i] % divisor == 0 {
                        nums[i] / divisor
                } 
                    else {
                        nums[i]    
                }
            }
            curr_mmc = curr_mmc * divisor;
        }
        else {
            divisor += 1;
        }
   }

   println!("mmc is {}", curr_mmc);
}

fn can_divide_something (nums: [i32; RANGE], divisor: i32) -> bool {
    for num in nums {
        if num % divisor == 0 {
            return true;
        }
    }

    return false;
}

fn is_all_one (nums: [i32; RANGE]) -> bool {
    for num in nums {
        if num != 1 {
            return false;
        }
    }

    return true;
}
