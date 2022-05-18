fn main() {
    let mut num: i64 = 600851475143;
    let mut divisor = 2;
    let mut greatest_divisor = 0;

    while num != 1 {
        if num % divisor == 0 {
            num = num / divisor;
            greatest_divisor = divisor;
        }

        divisor += 1;
    }
    println!("{}", greatest_divisor);
}
