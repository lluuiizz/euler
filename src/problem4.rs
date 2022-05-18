fn main() {
    println!("The greatest palyndrome is {}", greatest_palyndrome_up_to_n(2));
}


fn count_digits (num: i32) -> u32 {
    let mut count = 0;

    while num / 10_i32.pow(count) >= 1 {
        count += 1;
    }

    return count;
}

fn reverse_num (num: i32, length: u32) -> i32 {
    let mut reversed_num: i32 = 0;

    if length == 1 {return num};

    for i in 0..length {
        reversed_num += (num / 10_i32.pow(i) % 10) * 10_i32.pow(length - i - 1);
    }

    return reversed_num;

}

fn greatest_palyndrome_up_to_n (n: u32) -> i32 {
    let mut first_factor: i32 = 10_i32.pow(n - 1);
    let mut second_factor: i32 = 10_i32.pow(n - 1);
    let mut greatest_palyndrome = 0;
    
    while second_factor < 10_i32.pow(n) && first_factor < 10_i32.pow(n) {
        let resulted_num = first_factor * second_factor;
        let digits_count = count_digits(resulted_num);

        if reverse_num(resulted_num, digits_count) == resulted_num && 
            greatest_palyndrome <  resulted_num {
   
            greatest_palyndrome = resulted_num;
            
        }
        
        if second_factor == 10_i32.pow(n) - 1 {
            second_factor = 10_i32.pow(n - 1);
            first_factor += 1;
        } 
        else {
            second_factor += 1;
        }
    }

    return greatest_palyndrome;

}
