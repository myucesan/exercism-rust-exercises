pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 && year % 400 == 0 {
        true;
    } else if year % 100 == 0 {
        false;
    } else {
        true;
    }



    false
}