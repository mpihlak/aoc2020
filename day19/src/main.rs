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

fn match_rule(rules: &Vec<Rule>, rule_num: usize, text: &str) -> Option<usize> {
    for pattern in rules[rule_num].match_alternatives.iter() {
        let mut match_text = text;
        let mut match_count = 0;
        let mut match_len = 0;
        for m in pattern.iter() {
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

#[allow(dead_code)]
fn test_rules() {
    let rules_str = 
"0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";

    let rules: Vec<Rule> = rules_str.split('\n')
        .map(|s| Rule::from_str(s))
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

    let rules: Vec<Rule> = rules_str.split('\n')
        .map(|s| Rule::from_str(s))
        .collect();

    assert_eq!(Some(6), match_rule(&rules, 0, "ababbb"));
    assert_eq!(None, match_rule(&rules, 0, "bababa"));
}

fn main() {
    test_rules();
    test_rules2();

    let input_data = read_input_data();
    let mut split = input_data.split("\n\n");
    let rules_str = split.next().unwrap();
    let messages_str = split.next().unwrap();

    let mut rules: Vec<Rule> = rules_str.split('\n')
        .map(|s| Rule::from_str(s))
        .collect();
    rules.sort_by(|a,b| a.rule_number.cmp(&b.rule_number));

    let messages: Vec<String> = messages_str.split('\n')
        .map(|s| s.to_string())
        .collect();

    let mut count_matches = 0;
    for msg in messages.iter() {
        if let Some(len) = match_rule(&rules, 0, msg) {
            if len == msg.len() {
                count_matches += 1;
            }
        }
    }

    println!("Stage 1: answer = {}", count_matches);
}
