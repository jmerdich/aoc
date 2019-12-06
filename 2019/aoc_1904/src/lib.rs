#![allow(dead_code)]
/*
--- Day 4: Secure Container ---
You arrive at the Venus fuel depot only to discover it's protected by a
password. The Elves had written the password on a sticky note, but someone
threw it out.

However, they do remember a few key facts about the password:

It is a six-digit number. The value is within the range given in your puzzle
input. Two adjacent digits are the same (like 22 in 122345). Going from left
to right, the digits never decrease; they only ever increase or stay the same
(like 111123 or 135679).

Other than the range rule, the following are true:

111111 meets these criteria (double 11, never decreases). 223450 does not
meet these criteria (decreasing pair of digits 50). 123789 does not meet
these criteria (no double).

How many different passwords within the range given in your puzzle input meet
these criteria?
*/

fn predicate(input: &str) -> bool {
    const REQ_DIGITS: usize = 6;
    let mut found_double: bool = input.len() < REQ_DIGITS - 2;
    let mut chars = input.chars();
    let mut prev_num = chars.next().unwrap().to_digit(10).unwrap();
    for this_char in chars {
        let this_num = this_char.to_digit(10).unwrap();
        if this_num < prev_num {
            return false;
        }
        if this_num == prev_num {
            found_double = true;
        }
        prev_num = this_num;
    }
    found_double
}
fn predicate1b(input: &str) -> bool {
    let mut found_double: bool = false; // ignore < 6 digits...

    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for (i, num) in nums[1..].iter().enumerate() {
        let num = *num;
        let i = i + 1; // We start at 1...
        if num < nums[i - 1] {
            return false;
        }
        if num == nums[i - 1] {
            found_double = true;
        }
    }
    found_double
}

pub fn aoc4a(start: u32, end: u32) -> usize {
    (start..=end)
        .filter(|i| predicate1b(&i.to_string()))
        .count()
}

/*

An Elf just remembered one more important detail: the two adjacent matching
digits are not part of a larger group of matching digits.

Given this additional criterion, but still ignoring the range rule, the
following are now true:

112233 meets these criteria because the digits never decrease and all
repeated digits are exactly two digits long. 123444 no longer meets the
criteria (the repeated 44 is part of a larger group of 444). 111122 meets the
criteria (even though 1 is repeated more than twice, it still contains a
double 22).

How many different passwords within the range given in your puzzle input meet
all of the criteria?
*/
fn predicate2(input: &str) -> bool {
    let mut consecutive_count = 0;
    let mut retval = false;

    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for (i, num) in nums[1..].iter().enumerate() {
        let num = *num;
        let i = i + 1; // We start at 1...
        if num < nums[i - 1] {
            return false;
        }
        if num == nums[i - 1] {
            consecutive_count += 1;
        } else if consecutive_count != 0 {
            if consecutive_count == 1 {
                retval = true;
            }
            consecutive_count = 0
        }
    }
    if consecutive_count == 1 {
        retval = true;
    }
    retval
}

pub fn aoc4b(start: u32, end: u32) -> usize {
    (start..=end).filter(|i| predicate2(&i.to_string())).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc4a_mysamples() {
        assert_eq!(predicate("000123"), true);
        assert_eq!(predicate("321000"), false);
        assert_eq!(predicate("123456"), false);
        assert_eq!(predicate("111111"), true);
        assert_eq!(predicate("223450"), false);
        assert_eq!(predicate("123789"), false);
    }

    #[test]
    fn aoc4a_mysamples2() {
        assert_eq!(predicate1b("000123"), true);
        assert_eq!(predicate1b("321000"), false);
        assert_eq!(predicate1b("123456"), false);
        assert_eq!(predicate1b("111111"), true);
        assert_eq!(predicate1b("223450"), false);
        assert_eq!(predicate1b("123789"), false);
    }

    #[test]
    fn aoc4a_prob() {
        assert_eq!(aoc4a(128392, 643281), 2050);
    }

    #[test]
    fn aoc4b_mysamples() {
        assert_eq!(predicate2("124444"), false);
        assert_eq!(predicate2("112233"), true);
        assert_eq!(predicate2("223334"), true);
        assert_eq!(predicate2("111122"), true);
        assert_eq!(predicate2("123344"), true);
        assert_eq!(predicate2("000000"), false);
        assert_eq!(predicate2("321"), false);
        assert_eq!(predicate2("321000"), false);
        assert_eq!(predicate2("123444"), false);
        assert_eq!(predicate2("432100"), false);
        assert_eq!(predicate2("123456"), false);
    }

    #[test]
    fn aoc4b_prob() {
        assert_eq!(aoc4b(128392, 643281), 1390);
    }
}
