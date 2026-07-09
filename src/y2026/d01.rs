pub fn part1(input: &str) -> String {
    let nums = input.lines().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let sum: i32 = nums.iter().filter_map(|&n| if n < 60 { Some(60-n) } else { None }).sum();
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let nums = input.lines().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let sum: i32 = nums.iter().filter_map(|&n| if n < 60 { Some(60-n) } else { Some((n-60)*5) }).sum();
    sum.to_string()
}

pub fn part3(input: &str) -> String {
    let nums = input.lines().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let left = &nums[..nums.len()/2];
    let right = &nums[nums.len()/2..];
    let zipped = left.iter().zip(right.iter());
    let sum = zipped.filter_map(|(&l, &r)| if l < r { Some(r-l) } else { Some((l-r)*5) }).sum::<i32>();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "41\n87\n93\n104\n46\n102\n65\n105\n81\n36\n66\n46\n60\n65\n64\n64\n61\n73\n55\n69";
        assert_eq!(part1(input), "76");
    }

    #[test]
    fn test_part2() {
        let input = "41\n87\n93\n104\n46\n102\n65\n105\n81\n36\n66\n46\n60\n65\n64\n64\n61\n73\n55\n69";
        assert_eq!(part2(input), "1371");
    }

    #[test]
    fn test_part3() {
        let input = "41\n87\n93\n104\n46\n102\n65\n105\n81\n36\n66\n46\n60\n65\n64\n64\n61\n73\n55\n69";
        assert_eq!(part3(input), "1141");
    }
}
