fn main() {
    let mut x = 1;
    let mut y = 1;
    let mut sum = 0;
    

    while y < 90 {
        if y % 2 == 0 {
            sum += y;
        }
        let curr_y = y;
        y = x + y;
        x = curr_y;
    }
    println!("{}", sum);
}
