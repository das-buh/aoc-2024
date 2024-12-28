use aoc::FxHashMap;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut circuit = parse_input(input);
    let mut output = 0;

    for i in 0..64 {
        let wire = op_wire(b'z', i);
        if !circuit.contains_key(&wire) {
            break;
        }
        let value = eval(wire, &mut circuit);
        output |= (value as u64) << i;
    }

    output
}

fn eval(wire: Wire, circuit: &mut FxHashMap<Wire, Gate>) -> bool {
    let value = match circuit[&wire] {
        Gate::Op(op, lhs, rhs) => {
            let (lhs, rhs) = (eval(lhs, circuit), eval(rhs, circuit));
            match op {
                Op::And => lhs & rhs,
                Op::Or => lhs | rhs,
                Op::Xor => lhs ^ rhs,
            }
        }
        Gate::Value(value) => value,
    };
    *circuit.get_mut(&wire).unwrap() = Gate::Value(value);
    value
}

fn two(input: &str) -> Vec<String> {
    let circuit = parse_input(input);
    let mut bad = Vec::new();

    let z_top = *circuit.keys().filter(|wire| wire[0] == b'z').max().unwrap();

    for (wire, gate) in &circuit {
        let (wire, gate) = (*wire, *gate);

        if is_z(wire) && !is_xor(gate) && wire != z_top {
            bad.push(wire);
        }

        if !is_z(wire) && !recv_inputs(gate) && is_xor(gate) {
            bad.push(wire);
        }

        if recv_inputs(gate) && !recv_first(gate) {
            if is_xor(gate) && !leads_into(wire, &circuit).any(|op| op == Op::Xor) {
                bad.push(wire);
            }

            if is_and(gate) && !leads_into(wire, &circuit).all(|op| op == Op::Or) {
                bad.push(wire);
            }
        }
    }

    bad.sort();
    bad.dedup();

    bad.into_iter()
        .map(|w| String::from_utf8(w.to_vec()).unwrap())
        .collect()
}

fn is_z(wire: Wire) -> bool {
    wire[0] == b'z'
}

fn is_and(gate: Gate) -> bool {
    matches!(gate, Gate::Op(Op::And, ..))
}

fn is_xor(gate: Gate) -> bool {
    matches!(gate, Gate::Op(Op::Xor, ..))
}

fn operands(gate: Gate) -> Option<(Wire, Wire)> {
    match gate {
        Gate::Op(_, lhs, rhs) => Some((lhs, rhs)),
        Gate::Value(_) => None,
    }
}

fn recv_inputs(gate: Gate) -> bool {
    operands(gate).is_some_and(|(lhs, rhs)| {
        lhs[0] == b'x' && rhs[0] == b'y' || lhs[0] == b'y' && rhs[0] == b'x'
    })
}

fn recv_first(gate: Gate) -> bool {
    operands(gate).is_some_and(|(lhs, rhs)| {
        let (x0, y0) = (*b"x00", *b"y00");
        lhs == x0 && rhs == y0 || lhs == y0 && rhs == x0
    })
}

fn leads_into(wire: Wire, circuit: &FxHashMap<Wire, Gate>) -> impl Iterator<Item = Op> + use<'_> {
    circuit.iter().filter_map(move |(_, gate)| match *gate {
        Gate::Op(op, lhs, rhs) => (lhs == wire || rhs == wire).then_some(op),
        _ => None,
    })
}

type Wire = [u8; 3];

#[derive(Clone, Copy)]
enum Gate {
    Op(Op, Wire, Wire),
    Value(bool),
}

#[derive(Clone, Copy, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

fn parse_input(input: &str) -> FxHashMap<Wire, Gate> {
    use aoc::parse::*;

    let mut lines = input.lines();
    let mut circuit = FxHashMap::default();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let ((wire, value), _) = seq!(word, ": ", uint)(line);
        let wire = str_to_wire(wire);
        circuit.insert(wire, Gate::Value(value != 0));
    }

    for line in lines {
        let ((lhs, gate, rhs, wire), _) = seq!(word, " ", word, " ", word, " -> ", word)(line);
        let (lhs, rhs) = (str_to_wire(lhs), str_to_wire(rhs));
        let op = match gate {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!(),
        };
        circuit.insert(str_to_wire(wire), Gate::Op(op, lhs, rhs));
    }

    circuit
}

fn str_to_wire(s: &str) -> [u8; 3] {
    s.as_bytes().try_into().unwrap()
}

fn op_wire(op: u8, i: usize) -> [u8; 3] {
    [op, i as u8 / 10 + b'0', i as u8 % 10 + b'0']
}
