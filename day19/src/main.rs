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

fn match_rule(rules: &HashMap<usize,Rule>, rule_num: usize, text: &str) -> Option<usize> {
    let rule = rules.get(&rule_num).unwrap();

    for pattern in rule.match_alternatives.iter() {
        let mut match_text = text;
        let mut match_count = 0;
        let mut match_len = 0;

        for m in pattern.iter() {
            if match_text.is_empty() {
                break;
            }

            match m {
                Matcher::Literal(c) => {
                    if match_text.chars().next().unwrap() == *c {
                        match_text = &match_text[1..];
                        match_count += 1;
                        match_len += 1;
                    } else {
                        break;
                    }
                },
                Matcher::Indirect(rule) => {
                    if let Some(len) = match_rule(&rules, *rule, &match_text) {
                        match_text = &match_text[len..];
                        match_count += 1;
                        match_len += len;
                    } else {
                        break;
                    }
                }
            }
        }

        if match_count == pattern.len() {
            return Some(match_len);
        }
    }

    None
}

fn match_rule_debug(
    rules: &HashMap<usize, Rule>,
    rule_num: usize,
    text: &str,
    level: usize
) -> Vec<usize> {
    let rule = rules.get(&rule_num).unwrap();
    let mut result = Vec::new();
    let _debug = false;

    if _debug {
        println!("{}START: try match rule {}: {}", " ".repeat(level), rule_num, text);
    }

    for pattern in rule.match_alternatives.iter() {
        let mut matched_texts = vec![(text, 0)];

        for m in pattern.iter() {
            if _debug {
                println!("{}rule: {:?} match_texts: {:?}", " ".repeat(level), m, matched_texts);
            }
            let mut things_to_consider = Vec::new();

            while let Some((match_text, match_len)) = matched_texts.pop() {
                if match_text.is_empty() {
                    // Exhausted the text before the pattern. No result.
                    continue;
                }

                match m {
                    Matcher::Literal(c) => {
                        if match_text.chars().next().unwrap() == *c {
                            if _debug {
                                println!("{}matched literal {}", " ".repeat(level), *c);
                            }
                            things_to_consider.push((&match_text[1..], 1));
                        } else {
                            break;
                        }
                    },
                    Matcher::Indirect(rule) => {
                        // Here the rule might contain multiple matching alternative sub-rules.
                        // If this happens, multiple results will be returned and we need to try each
                        // of them.
                        for len in match_rule_debug(&rules, *rule, &match_text, level+1) {
                            if _debug {
                                println!("{}subrule matched {} chars, consider {}",
                                    " ".repeat(level), len, &match_text[len..]);
                            }
                            things_to_consider.push((&match_text[len..], match_len+len));
                        }
                    }
                }
            }

            if _debug {
                println!("{}reassigning: {:?}", " ".repeat(level), things_to_consider);
            }
            matched_texts = things_to_consider;
        }

        for (match_text, match_len) in matched_texts.iter() {
            if _debug {
                println!("{}matched rule {}, remaining: {} ({})", " ".repeat(level), rule_num, match_text, match_len);
            }
            result.push(*match_len);
        }
    }

    if _debug {
        println!("{}patterns exhausted: returning {:?}", " ".repeat(level), result);
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

    assert_eq!(Some(1), match_rule(&rules, 1, "a"));
    assert_eq!(None, match_rule(&rules, 1, "b"));
    assert_eq!(Some(2), match_rule(&rules, 2, "ab"));
    assert_eq!(Some(2), match_rule(&rules, 2, "ba"));
    assert_eq!(None, match_rule(&rules, 2, "aa"));
    assert_eq!(None, match_rule(&rules, 2, "bb"));
    assert_eq!(Some(3), match_rule(&rules, 0, "aab"));
    assert_eq!(Some(3), match_rule(&rules, 0, "aba"));
    assert_eq!(None, match_rule(&rules, 0, "aaa"));
    assert_eq!(None, match_rule(&rules, 0, "abb"));
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

    assert_eq!(Some(6), match_rule(&rules, 0, "ababbb"));
    assert_eq!(None, match_rule(&rules, 0, "bababa"));
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

    assert_eq!(vec![0 as usize; 0], match_rule_debug(&rules, 0, "bb", 0));
    println!();
    assert_eq!(vec![2_usize], match_rule_debug(&rules, 0, "ba", 0));
    println!();
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

    let mut count_matches = 0;
    for msg in messages.iter() {
        if let Some(len) = match_rule(&rules, 0, msg) {
            if len == msg.len() {
                count_matches += 1;
            }
        }
    }

    println!("Stage 1: answer = {}", count_matches);

    rules.insert( 8, Rule::from_str("8: 42 | 42 8"));
    rules.insert(11, Rule::from_str("11: 42 31 | 42 11 31"));

    let mut count_matches = 0;
    for msg in messages.iter() {
        for len in match_rule_debug(&rules, 0, msg, 0) {
            if len == msg.len() {
                count_matches += 1;
                break;
            }
        }
    }

    println!("Stage 2: answer = {}", count_matches);
}
