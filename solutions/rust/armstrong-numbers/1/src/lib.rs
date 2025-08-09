pub fn is_armstrong_number(num: u64) -> bool {
    if num == 0 {
        return true;
    }

    // Suggested by clippy
    if (1..=9).contains(&num) {
        return true;
    }

    let mut digits_of_num: Vec<u64> = Vec::new();

    let mut num_copy = num;

    while num_copy != 0 {
        let digit = num_copy % 10;
        digits_of_num.push(digit);
        num_copy /= 10;
    }

    let num_digits_sum = digits_of_num
        .iter()
        .map(|d| d.pow(digits_of_num.len() as u32))
        .reduce(|acc, d_cubed| acc + d_cubed)
        .expect("Addition of powers of digits failed!");

    // return true if num_digits_sum == num
    num_digits_sum == num
}
