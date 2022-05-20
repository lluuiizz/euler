fn main() {
    let n: i32 = 100;
    
    let n_div_4: i32 = (n * 2 - 4) / 4;
    let mut sum_of_squares = (n.pow(2) + 1) * n/2;

    let mut i: i32 = 0;
    while n_div_4 - i != 0 {
        sum_of_squares -= 4 * (n_div_4 - i).pow(2);
        i += 1;

    }

    let square_of_the_sum:i32 = ((n + 1) * n/2).pow(2);

    let difference = square_of_the_sum - sum_of_squares;

    println!("The answear is {}", difference);
}
