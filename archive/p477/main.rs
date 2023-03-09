impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for bit in 0..32{
            let mut count0 = 0;
            let mut count1 = 0;
            for x in &nums {
                if x & 1 << bit > 0 { count1 += 1;}
                else { count0 += 1;}
            }
            res += count1 * count0;
        }
        res
    }
}

struct Solution;
fn main() {
//    let mut line = String::new();
//    std::io::stdin().read_line(&mut line);
//    let line = line[..line.len()-1].to_string();

    println!("{}", Solution::total_hamming_distance(vec![4, 14, 2]));
}
