use std::collections::HashMap;
use aoclib::*;

#[derive(Debug)]
enum Matcher {
    Literal(char),
    Indirect(usize),
}

#[derive(Debug)]
struct Rule {
    rule_number: usize,
    match_alternatives: Vec<Vec<Matcher>>,
}

impl Rule {
    fn from_str(rule_text: &str) -> Self {
        let mut s = rule_text.split(": ");
        let rule_number = s.next().unwrap().parse().unwrap(); 
        let rule_body = s.next().unwrap();
        
        let mut match_alternatives = Vec::new();
        for alt_rule in rule_body.split(" | ") {
            let mut matchers: Vec<Matcher> = Vec::new();
            for rule_matcher in alt_rule.split(' ') {
                let m = if rule_matcher.starts_with("\"") {
                    Matcher::Literal(rule_matcher[1..].chars().next().unwrap())
                } else {
                    Matcher::Indirect(rule_matcher.parse().unwrap())
                };
                matchers.push(m);
            }
            match_alternatives.push(matchers);
        }

        Rule { rule_number, match_alternatives }
    }
}

fn match_rule(rules: &HashMap<usize, Rule>, rule_num: usize, text: &str) -> bool {
    match_rule_aux(rules, rule_num, text)
        .iter()
        .find(|x| **x == text.len())
        .is_some()
}

fn match_rule_aux(rules: &HashMap<usize, Rule>, rule_num: usize, text: &str) -> Vec<usize> {
    let rule = rules.get(&rule_num).unwrap();
    let mut result = Vec::new();

    for pattern in rule.match_alternatives.iter() {
        let mut matched_texts = vec![(text, 0)];

        for m in pattern.iter() {
            let mut matches_to_consider = Vec::new();

            while let Some((match_text, match_len)) = matched_texts.pop() {
                if match_text.is_empty() {
                    // Exhausted the text before the pattern. No result.
                    continue;
                }

                match m {
                    Matcher::Literal(c) => {
                        if match_text.chars().next().unwrap() == *c {
                            matches_to_consider.push((&match_text[1..], 1));
                        } else {
                            break;
                        }
                    },
                    Matcher::Indirect(rule) => {
                        // Here the rule might contain multiple matching alternative sub-rules.
                        // If this happens, multiple results will be returned and we need to try each
                        // of them.
                        for len in match_rule_aux(&rules, *rule, &match_text) {
                            matches_to_consider.push((&match_text[len..], match_len+len));
                        }
                    }
                }
            }

            matched_texts = matches_to_consider;
        }

        for (_, match_len) in matched_texts.iter() {
            // Pushing all matches to result, and have the caller figure out which
            // are the correct ones. It's annoying to do it here, because of partial
            // matches by literals.
            result.push(*match_len);
        }
    }

    result
}

#[allow(dead_code)]
fn test_rules() {
    let rules_str = 
"0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";

    let rules: HashMap<usize, Rule> = rules_str.split('\n')
        .map(|s| { let r = Rule::from_str(s); ( r.rule_number, r ) } )
        .collect();

    assert_eq!(true, match_rule(&rules, 1, "a"));
    assert_eq!(false, match_rule(&rules, 1, "b"));
    assert_eq!(true, match_rule(&rules, 2, "ab"));
    assert_eq!(true, match_rule(&rules, 2, "ba"));
    assert_eq!(false, match_rule(&rules, 2, "aa"));
    assert_eq!(false, match_rule(&rules, 2, "bb"));
    assert_eq!(true, match_rule(&rules, 0, "aab"));
    assert_eq!(true, match_rule(&rules, 0, "aba"));
    assert_eq!(false, match_rule(&rules, 0, "aaa"));
    assert_eq!(false, match_rule(&rules, 0, "abb"));
}

#[allow(dead_code)]
fn test_rules2() {
    let rules_str = 
"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"";

    let rules: HashMap<usize,Rule> = rules_str.split('\n')
        .map(|s| { let r = Rule::from_str(s); ( r.rule_number, r ) } )
        .collect();

    assert_eq!(true, match_rule(&rules, 0, "ababbb"));
    assert_eq!(false, match_rule(&rules, 0, "bababa"));
}

#[allow(dead_code)]
fn test_rules3() {
    let rules_str = r#"
0: 1 | 2 0
1: "a"
2: "b"
"#;

    let rules: HashMap<usize,Rule> = rules_str.trim_start().trim_end().split('\n')
        .map(|s| { let r = Rule::from_str(s); ( r.rule_number, r ) } )
        .collect();

    assert_eq!(false, match_rule(&rules, 0, "bb"));
    assert_eq!(true, match_rule(&rules, 0, "ba"));
}

fn main() {
    let input_data = read_input_data();
    let mut split = input_data.split("\n\n");
    let rules_str = split.next().unwrap();
    let messages_str = split.next().unwrap();

    let mut rules: HashMap<usize, Rule> = rules_str.split('\n')
        .map(|s| { let r = Rule::from_str(s); ( r.rule_number, r ) } )
        .collect();

    let messages: Vec<String> = messages_str.split('\n')
        .map(|s| s.to_string())
        .collect();

    test_rules();
    test_rules2();
    test_rules3();

    let count_matches = messages.iter()
        .map(|msg| match_rule(&rules, 0, msg))
        .filter(|matches| *matches)
        .count();
    println!("Stage 1: answer = {}", count_matches);

    rules.insert( 8, Rule::from_str("8: 42 | 42 8"));
    rules.insert(11, Rule::from_str("11: 42 31 | 42 11 31"));

    let count_matches = messages.iter()
        .map(|msg| match_rule(&rules, 0, msg))
        .filter(|matches| *matches)
        .count();
    println!("Stage 2: answer = {}", count_matches);
}
