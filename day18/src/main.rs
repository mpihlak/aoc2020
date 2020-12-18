use aoclib::*;

fn parenthesized_expr(expr: &str) -> (&str, usize) {
    let mut len = 0;
    let mut level = 1;
    let mut chars = expr[1..].chars();

    while level > 0 {
        len += 1;
        match chars.next() {
            Some('(') => level += 1,
            Some(')') => level -= 1,
            Some(_) => {},
            _ => panic!("invalid expression"),
        }
    }
    (&expr[..len+1], len+1)
}

fn number_or_expression<F>(expr: &str, eval: F) -> (i64, &str)
    where F: Fn(&str) -> i64
{
    if expr.starts_with('(') {
        let (pe, len) = parenthesized_expr(&expr);
        let remainder = &expr[len..];
        (eval(&pe[1..len-1]), remainder.trim_start())
    } else {
        let digits = expr.split(' ').next().unwrap();
        let remainder = expr.strip_prefix(digits).unwrap();
        (digits.parse().unwrap(), remainder.trim_start())
    }

}

fn operator(expr: &str) -> (&str, &str) {
    let op = expr.trim_start().split(' ').next().unwrap();
    let remainder = expr.trim_start().strip_prefix(op).unwrap();
    (op, remainder.trim_start())
}

fn eval(expr: &str) -> i64 {
    // Just an integer, return literal value
    if let Ok(val) = expr.trim_start().parse::<i64>() {
        return val;
    }

    let (mut left, mut expr) = number_or_expression(&expr, eval);
    while expr != "" {
        let (op, remainder) = operator(&expr);
        let (right, remainder) = number_or_expression(&remainder, eval);

        left = match op {
            "+" => left + right,
            "*" => left * right,
            other => panic!("unknown operator: {}", other),
        };
        expr = remainder;
    }

    left
}

fn eval_with_prio(expr: &str) -> i64 {
    if let Ok(val) = expr.trim_start().parse::<i64>() {
        return val;
    }

    let (val, mut expr) = number_or_expression(&expr, eval_with_prio);
    let mut res = vec![val];

    while expr != "" {
        let (op, remainder) = operator(&expr);
        let (right, remainder) = number_or_expression(&remainder, eval_with_prio);

        match op {
            "+" => {
                let left = res.pop().unwrap();
                res.push(left + right);
            },
            "*" => res.push(right),
            other => panic!("unknown operator: {}", other),
        };

        expr = remainder;
    }

    res.iter().product()
}


fn main() {
    assert_eq!(71, eval("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(26, eval("2 * 3 + (4 * 5)"));
    assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    assert_eq!(13632, eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));

    let input_data = read_input_data();

    let answer: i64 = input_data
        .split('\n')
        .map(|x| eval(x))
        .sum();
    println!("Stage 1: answer = {}", answer);

    assert_eq!(231, eval_with_prio("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(51, eval_with_prio("1 + (2 * 3) + (4 * (5 + 6))"));
    assert_eq!(46, eval_with_prio("2 * 3 + (4 * 5)"));
    assert_eq!(1445, eval_with_prio("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    assert_eq!(669060, eval_with_prio("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    assert_eq!(23340, eval_with_prio("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));

    let answer: i64 = input_data
        .split('\n')
        .map(|x| eval_with_prio(x))
        .sum();
    println!("Stage 2: answer = {}", answer);
}
