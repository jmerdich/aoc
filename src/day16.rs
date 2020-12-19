#![allow(unused_variables, dead_code)]

use itertools::Itertools;
use std::collections::HashMap;
use z3::ast;
use z3::ast::Ast;
use z3::{Sort, Symbol};

#[derive(Debug, Clone)]
pub struct Content {
    fields: HashMap<String, Vec<(u32, u32)>>,
    my_ticket: Vec<u32>,
    others: Vec<Vec<u32>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Content {
    let lines: Vec<&str> = input.lines().collect_vec();

    let split1 = lines.iter().find_position(|l| l.is_empty()).unwrap().0;
    let split2 = lines[split1 + 1..]
        .iter()
        .find_position(|l| l.is_empty())
        .unwrap()
        .0
        + split1
        + 1;

    let fields = lines[..split1]
        .iter()
        .map(|l| {
            let (name, ranges) = l.split(": ").collect_tuple().unwrap();
            let ranges: Vec<(u32, u32)> = ranges
                .split(" or ")
                .map(|r| {
                    r.split('-')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect();
            (name.to_string(), ranges)
        })
        .collect();

    let my_ticket = lines[split1 + 2]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let others = lines[split2 + 2..]
        .iter()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Content {
        fields,
        my_ticket,
        others,
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Content) -> u32 {
    let mut err_vals: Vec<u32> = Vec::new();

    for ticket in &input.others {
        for val in ticket {
            let bad = input
                .fields
                .values()
                .flatten()
                .all(|(start, end)| val < start || val > end);

            if bad {
                err_vals.push(*val);
            }
        }
    }

    err_vals.iter().copied().sum()
}

pub fn get_all_opts<'a>(
    fields: &'a HashMap<String, Vec<(u32, u32)>>,
    valid_tickets: &[&Vec<u32>],
) -> Vec<Vec<&'a str>> {
    let mut field_options: Vec<Vec<&str>> = Vec::new();

    for idx in 0..valid_tickets[0].len() {
        let mut options: Vec<&str> = Vec::new();

        for (name, ranges) in fields {
            let is_possible = valid_tickets.iter().all(|t| {
                ranges.iter().any(|(s, e)| t[idx] >= *s && t[idx] <= *e )
            });
            if is_possible {
                options.push(name);
            }
        }

        assert!(!options.is_empty());

        field_options.push(options);
    }

    field_options
}

pub fn get_valid_tickets(content: &Content) -> Vec<&Vec<u32>> {
    let all_ranges: Vec<(u32, u32)> = content.fields.values().flatten().copied().collect();
    let valid_tickets: Vec<&Vec<u32>> = content
        .others
        .iter()
        .filter(|t| {
            !t.iter()
                .any(|v| all_ranges.iter().all(|(s, e)| v < s || v > e))
        })
        .collect();

    assert!(valid_tickets.iter().map(|t| t.len()).all_equal());
    assert!(!valid_tickets.is_empty());

    valid_tickets
}

pub fn solve_for_fields<'a>(
    all_options: &[&'a str],
    field_options: &[Vec<&'a str>],
) -> Vec<&'a str> {
    let z3_conf = z3::Config::new();
    let z3_ctx = z3::Context::new(&z3_conf);

    let syms = all_options
        .iter()
        .map(|s| Symbol::String(s.to_string()))
        .collect::<Vec<Symbol>>();

    let (field_t, field_consts, field_checkers) =
        Sort::enumeration(&z3_ctx, Symbol::String("field_t".to_string()), &syms);

    let slots: Vec<ast::Datatype> = (0..field_options.len())
        .map(|i| ast::Datatype::new_const(&z3_ctx, i as u32, &field_t))
        .collect();

    let checker_map: HashMap<String, z3::FuncDecl> = all_options
        .iter()
        .map(|s| s.to_string())
        .zip(field_checkers)
        .collect();

    let solver = z3::Solver::new(&z3_ctx);

    solver.assert(&Ast::distinct(
        &z3_ctx,
        &slots.iter().collect::<Vec<&ast::Datatype>>(),
    ));

    for (slot, opts) in slots.iter().zip(field_options.iter()) {
        let preds: Vec<_> = opts
            .iter()
            .map(|name| checker_map[*name].apply(&[&ast::Dynamic::from_ast(slot)]))
            .map(|d| d.as_bool().unwrap())
            .collect();
        solver.assert(&ast::Bool::or(&z3_ctx, &preds.iter().collect::<Vec<_>>()));
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);

    let model = solver.get_model().unwrap();

    let results: Vec<_> = slots
        .iter()
        .map(|sl| model.eval(sl).unwrap())
        .map(|sym| {
            field_consts
                .iter()
                .position(|fc| fc.apply(&[]).as_datatype().unwrap() == sym)
                .unwrap()
        })
        .map(|i| all_options[i])
        .collect();
    results
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Content) -> u64 {
    let valid_tickets = get_valid_tickets(&input);

    let field_options = get_all_opts(&input.fields, &valid_tickets);

    let field_names = solve_for_fields(
        &input.fields.keys().map(|s| s.as_str()).collect::<Vec<_>>(),
        &field_options,
    );

    let mut out = 1;
    for (idx, name) in field_names.iter().enumerate() {
        if name.starts_with("departure") {
            out *= input.my_ticket[idx] as u64;
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    const INPUT: &str = include_str!("../input/2020/day16.txt");

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        dbg!(&content);
        assert_eq!(solve_part1(&content), 71);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        //assert_eq!(solve_part2(&content), 0);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 24980);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 809376774329);
    }
}
