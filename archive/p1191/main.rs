impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut hb = 0;
        let mut tmp = num;
        while tmp > 0 {
            tmp /= 2;
            hb += 1;
        }

        2_i32.pow(hb) - num - 1
    }
}

struct Solution;
fn main() {
    println!("Hello, world!");
    print("{}", Solution::find_complement(5));
}
