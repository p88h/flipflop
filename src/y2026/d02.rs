pub fn move_it(pos: usize, c: char, length: usize) -> usize {
    if c == '>' {
        (pos + 1) % length
    } else if c == '<' {
        (pos + length - 1) % length
    } else {
        pos
    }
}

pub fn max_score(temp: &[usize]) -> usize {
    let mut max = 0;
    for i in 0..temp.len() {
        if temp[i] > temp[max] {
            max = i;
        }
    }
    (max + 1) * temp[max]
}

pub fn part1(input: &str) -> String {
    let length = 100;
    let mut temp = vec![0; length];
    let mut pos = 0;
    for c in input.chars() {
        pos = move_it(pos, c, length);
        temp[pos] += 1;
    }
    max_score(&temp).to_string()
}

pub fn part2(input: &str) -> String {
    let mut rpos = 0;
    let mut wpos = 0;
    let chars: Vec<char> = input.chars().collect();
    let length = chars.len();
    let mut sum = 0;
    for i in 0..length {
        let c = chars[i];
        let d = chars[length - i - 1];
        rpos = move_it(rpos, c, 100);
        wpos = move_it(wpos, d, 100);
        if rpos == wpos {
            sum += 1;
        }
    }
    sum.to_string()
}

pub fn part3(input: &str) -> String {
    let mut temp = vec![0; 100];
    let mut rpos = 0;
    let mut wpos = 0;
    let chars: Vec<char> = input.chars().collect();
    let length = chars.len();
    for i in 0..length {
        let c = chars[i];
        let d = chars[length - i - 1];
        rpos = move_it(rpos, c, 100);
        wpos = move_it(wpos, d, 100);
        let pos = (rpos + 100 - wpos) % 100;
        temp[pos] += 1;
    }
    max_score(&temp).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "><>><<>><<<>>>>><><><><><>>>>>";
        assert_eq!(part1(input), "12");
    }

    #[test]
    fn test_part2() {
        let input = "><>><<>><<<>>>>><><><><><>>>>>";
        assert_eq!(part2(input), "3");
    }

    #[test]
    fn test_part3() {
        let input = "><>><<>><<<>>>>><><><><><>>>>>";
        assert_eq!(part3(input), "1358");
    }
}
