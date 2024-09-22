pub struct ReverseInt;

impl ReverseInt {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let mut x = x;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && pop > 7) {
                return 0;
            }
            if result < i32::MIN / 10 || (result == i32::MIN / 10 && pop < -8) {
                return 0;
            }
            result = result * 10 + pop;
        }
        result
    }
}
