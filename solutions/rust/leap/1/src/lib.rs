pub fn is_leap_year(year: u64) -> bool {

    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                // Divisible by 100 and 400
                return true
            }
            // Divisible by 100, but not by 400
            return false
        }
        // Divisible by 4. Not divisible by 100
        return true
    }
    // Not divisible by 4
    return false
}
