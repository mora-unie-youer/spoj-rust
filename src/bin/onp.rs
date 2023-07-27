use std::io::BufRead;

/**
 * ONP - Transform the Expression
 *
 * Transform the algebraic expression with brackets into RPN form (Reverse Polish Notation).
 * Two-argument operators: +, -, *, /, ^ (priority from the lowest to the highest), brackets ( ).
 * Operands: only letters: a,b,...,z. Assume that there is only one RPN form (no expressions like a*b*c).
 *
 * Input
 *   t [the number of expressions <= 100]
 *   expression [length <= 400]
 *   [other expressions]
 * Text grouped in [ ] does not appear in the input file.
 *
 * Output
 * The expressions in RPN form, one per line.
 *
 * Example:
 *   Input:
 *   3
 *   (a+(b*c))
 *   ((a+b)*(z+x))
 *   ((a+t)*((b+(a+c))^(c+d)))
 *
 *   Output:
 *   abc*+
 *   ab+zx+*
 *   at+bac++cd+^*
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        println!("{}", convert_to_rpn(&line));
    }
}

fn operator_priority(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn convert_to_rpn(expr: &str) -> String {
    let mut stack: Vec<char> = vec![];

    let mut rpn_expr = String::new();
    for ch in expr.chars() {
        if ch.is_alphabetic() {
            rpn_expr.push(ch);
        } else if ch == '(' {
            stack.push('(');
        } else if ch == ')' {
            while let Some(ch) = stack.pop() {
                if ch == '(' {
                    break;
                }

                rpn_expr.push(ch);
            }
        } else {
            let priority = operator_priority(ch);

            while stack
                .last()
                .map(|&ch| operator_priority(ch) >= priority)
                .unwrap_or(false)
            {
                rpn_expr.push(stack.pop().unwrap());
            }

            stack.push(ch);
        }
    }

    rpn_expr
}
