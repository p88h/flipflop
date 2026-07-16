use rayon::prelude::*;

pub fn part1(input: &str) -> String {
    let mut best = "".to_string();
    let mut max_score = 0;
    for word in input.lines() {
        let mut score = 0;
        if word.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if word.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if word.chars().any(|c| c.is_digit(10)) {
            score += 1;
        }
        score *= word.len();
        if score > max_score {
            max_score = score;
            best = word.to_string();
        }
    }
    best
}

fn score_word(word: &str) -> usize {
    let mut score = 0;
    let num_digits = word.chars().filter(|c| c.is_digit(10)).count();
    let num_7 = word.chars().filter(|c| *c == '7').count();
    if num_digits > 0 {
        score += 1;
    }
    if num_digits == num_7 && num_7 > 0 {
        score += 7;
    }
    if word.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }
    if word.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }
    let mut prev = '\0';
    let mut consecutive_count = 1;
    let mut max_consecutive = 1;
    for c in word.chars() {
        if c == prev {
            consecutive_count += 1;
        } else {
            consecutive_count = 1;
        }
        if consecutive_count > max_consecutive {
            max_consecutive = consecutive_count;
        }
        prev = c;
    }
    if max_consecutive >= 3 {
        score += max_consecutive * max_consecutive;
    }
    let colors = ["red", "green", "blue"];
    for color in colors.iter() {
        if word.contains(color) {
            score *= 3;
            break;
        }
    }
    score * word.len()
}

pub fn part2(input: &str) -> String {
    let mut best = "".to_string();
    let mut max_score = 0;
    for word in input.lines() {
        let score = score_word(word);
        if score > max_score {
            max_score = score;
            best = word.to_string();
        }
    }
    best
}

pub fn part3(input: &str) -> String {
    let max_sum = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect::<Vec<char>>()
        .into_par_iter()
        .map(|suffix| {
            let mut sum = 0;
            for word in input.lines() {
                let mut new_word = word.to_string();
                new_word.push(suffix);
                let score = score_word(&new_word);
                sum += score;
            }
            sum
        })
        .max()
        .unwrap_or(0);
    max_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "aaaaa111\nKs3SDblu\neowcdredkcasdblu\nde333333\n7dedlblu\no3klll\n8ebluered\nDkoGreenD7\ngreen037";
        assert_eq!(part1(input), "DkoGreenD7");
    }

    #[test]
    fn test_part2() {
        let input = "aaaaa111\nKs3SDblu\neowcdredkcasdblu\nde333333\n7dedlblu\no3klll\n8ebluered\nDkoGreenD7\ngreen037";
        assert_eq!(part2(input), "de333333");
    }

    #[test]
    fn test_part3() {
        let input = "aaaaa111\nKs3SDblu\neowcdredkcasdblu\nde333333\n7dedlblu\no3klll\n8ebluered\nDkoGreenD7\ngreen037";
        assert_eq!(part3(input), "1453");
    }
}
