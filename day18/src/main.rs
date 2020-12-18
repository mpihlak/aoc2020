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

fn number_or_expression(expr: &str) -> (i64, &str) {
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

    let (mut left, mut expr) = number_or_expression(&expr);
    while expr != "" {
        let (op, remainder) = operator(&expr);
        let (right, remainder) = number_or_expression(&remainder);

        left = match op {
            "+" => left + right,
            "*" => left * right,
            other => panic!("unknown operator: {}", other),
        };
        expr = remainder;
    }

    left
}

fn main() {
    assert_eq!(71, eval("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(26, eval("2 * 3 + (4 * 5)"));
    assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    assert_eq!(13632, eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));

    let input_data = read_input_data();
    let sum: i64 = input_data
        .split('\n')
        .map(|x| eval(x))
        .sum();
    println!("Stage 1: answer = {}", sum);
}
