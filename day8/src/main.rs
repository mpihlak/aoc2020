use aoclib::*;

#[derive(Debug,Clone)]
struct Op {
    op: String,
    value: i32,
}

#[derive(Debug)]
enum ComputeResult {
    InfiniteLoop(i32),
    Completed(i32),
}

fn interpret(program: &Vec<Op>) -> ComputeResult {
    let mut pos = 0;
    let mut acc = 0;
    let mut visited: Vec<bool> = (0..program.len()).map(|_| false).collect();

    while pos < program.len() {
        if visited[pos] {
            return ComputeResult::InfiniteLoop(acc);
        }

        visited[pos] = true;
        let mut advance = 1;
        match program[pos].op.as_str() {
            "nop" => {},
            "acc" => acc += program[pos].value,
            "jmp" => advance = program[pos].value,
            other => panic!("invalid opcode: {}", other),
        }
        pos = (pos as i32 + advance) as usize;
    }

    ComputeResult::Completed(acc)
}

fn main() {
    let input_data = read_input_data();
    let mut program = Vec::new();
    for line in input_data.split("\n") {
        let mut it = line.split(" ");
        let op = it.next().unwrap().to_string();
        let value = it.next().unwrap().parse::<i32>().unwrap();
        program.push(Op { op, value });
    }

    if let ComputeResult::InfiniteLoop(acc) = interpret(&program) {
        println!("Stage 1: accumulator value = {:?}", acc);
    }

    for pos in 0..program.len() {
        let mut p = program.clone();

        match p[pos].op.as_str() {
            "jmp" => p[pos].op = "nop".to_string(),
            "nop" => p[pos].op = "jmp".to_string(),
            _ => {},
        }

        if let ComputeResult::Completed(acc) = interpret(&p) {
            println!("Stage 2: accumulator value = {:?}", acc);
            break;
        }
    }

}
