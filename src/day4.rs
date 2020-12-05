#![allow(unused_variables, dead_code)]

use regex::Regex;

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    static ref PASS_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref EYE_VALS: Vec<&'static str> = vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth");
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Id {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expire_year: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Id {
    fn is_trivially_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expire_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.is_trivially_valid() {
            return false;
        }

        if !(1920..=2002).contains(&self.birth_year.unwrap()){ 
            return false;
        }
        if !(2010..=2020).contains(&self.issue_year.unwrap()) {
            return false;
        }
        if !(2020..=2030).contains(&self.expire_year.unwrap()) {
            return false;
        }

        if !EYE_VALS.contains(&self.eye_color.as_ref().unwrap().as_str()) {
            return false;
        }

        if !PASS_RE.is_match(&self.passport_id.as_ref().unwrap()) {
            return false;
        }

        if !HAIR_RE.is_match(&self.hair_color.as_ref().unwrap()) {
            return false;
        }

        if let Some(caps) = HEIGHT_RE.captures(&self.height.as_ref().unwrap()) {
            let val: u32 = caps.get(1).unwrap().as_str().parse().unwrap();

            match caps.get(2).unwrap().as_str() {
                "in" => {
                    if !(59..=76).contains(&val) {
                        return false;
                    }
                }
                "cm" => {
                    if !(150..=193).contains(&val) {
                        return false;
                    }
                }
                _ => panic!(),
            }
        } else {
            return false;
        }

        true
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Id> {
    let mut ids: Vec<Id> = Vec::new();
    let mut cur_id: Id = Default::default();
    for l in input.lines() {
        if l.is_empty() {
            assert!(cur_id != Default::default());
            ids.push(cur_id.clone());
            cur_id = Default::default();
        }

        for pair_s in l.split_whitespace() {
            let pair: Vec<&str> = pair_s.split(':').collect();
            assert!(pair.len() == 2);
            let (key, val) = (pair[0], pair[1]);
            match key {
                "byr" => {
                    cur_id.birth_year = Some(val.parse().unwrap());
                }
                "iyr" => {
                    cur_id.issue_year = Some(val.parse().unwrap());
                }
                "eyr" => {
                    cur_id.expire_year = Some(val.parse().unwrap());
                }
                "hgt" => {
                    cur_id.height = Some(val.to_string());
                }
                "hcl" => {
                    cur_id.hair_color = Some(val.to_string());
                }
                "ecl" => {
                    cur_id.eye_color = Some(val.to_string());
                }
                "pid" => {
                    cur_id.passport_id = Some(val.to_string());
                }
                "cid" => {
                    cur_id.country_id = Some(val.to_string());
                }
                _ => panic!("Unexpected id field"),
            }
        }
    }
    if cur_id != Default::default() {
        ids.push(cur_id);
    }
    ids
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Id]) -> usize {
    input.iter().filter(|id| id.is_trivially_valid()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Id]) -> usize {
    input.iter().filter(|id| id.is_valid()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const EG_VALID_INPUT: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    const EG_INVALID_INPUT: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    const INPUT: &str = include_str!("../input/2020/day4.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 2);
    }
    #[test]
    fn eg_part2() {
        let orig_content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&orig_content), 2);
        let valid_content = input_generator(EG_VALID_INPUT);
        assert_eq!(solve_part2(&valid_content), 4);
        let invalid_content = input_generator(EG_INVALID_INPUT);
        assert_eq!(solve_part2(&invalid_content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 213);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 147);
    }
}
