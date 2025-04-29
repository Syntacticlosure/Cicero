use super::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinOp {
    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    I32Eq,
    I32Gt,
    I32Geq,
    I32Lt,
    I32Leq,
    I32And,
    I32Or,
    I32Xor,
    I32Not,

    I64Add,
    I64Sub,
    I64Mul,
    I64Div,
    I64Eq,
    I64Gt,
    I64Geq,
    I64Lt,
    I64Leq,
    I64And,
    I64Or,
    I64Xor,
    I64Not,

    U32Add,
    U32Sub,
    U32Mul,
    U32Div,
    U32Eq,
    U32Gt,
    U32Geq,
    U32Lt,
    U32Leq,
    U32And,
    U32Or,
    U32Xor,
    U32Not,

    U64Add,
    U64Sub,
    U64Mul,
    U64Div,
    U64Eq,
    U64Gt,
    U64Geq,
    U64Lt,
    U64Leq,
    U64And,
    U64Or,
    U64Xor,
    U64Not,
}

pub fn builtin_call<'a>(op: &'a BuiltinOp, args: Vec<Value<'a>>) -> Value<'a> {
    match op {
        BuiltinOp::I32Add => {
            if args.len() != 2 {
                panic!("I32Add: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 + v2),
                _ => panic!("I32Add: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Sub => {
            if args.len() != 2 {
                panic!("I32Sub: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 - v2),
                _ => panic!("I32Sub: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Mul => {
            if args.len() != 2 {
                panic!("I32Mul: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 * v2),
                _ => panic!("I32Mul: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Div => {
            if args.len() != 2 {
                panic!("I32Div: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 / v2),
                _ => panic!("I32Div: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Eq => {
            if args.len() != 2 {
                panic!("I32Eq: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 == v2),
                _ => panic!("I32Eq: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Gt => {
            if args.len() != 2 {
                panic!("I32Gt: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 > v2),
                _ => panic!("I32Gt: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Geq => {
            if args.len() != 2 {
                panic!("I32Geq: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 >= v2),
                _ => panic!("I32Geq: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Lt => {
            if args.len() != 2 {
                panic!("I32Lt: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 < v2),
                _ => panic!("I32Lt: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Leq => {
            if args.len() != 2 {
                panic!("I32Leq: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::Bool(v1 <= v2),
                _ => panic!("I32Leq: wrong type of arguments"),
            }
        }
        BuiltinOp::I32And => {
            if args.len() != 2 {
                panic!("I32And: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 & v2),
                _ => panic!("I32And: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Or => {
            if args.len() != 2 {
                panic!("I32Or: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 | v2),
                _ => panic!("I32Or: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Xor => {
            if args.len() != 2 {
                panic!("I32Xor: requires 2 arguments but recieves {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I32(v1), Value::I32(v2)) => Value::I32(v1 ^ v2),
                _ => panic!("I32Xor: wrong type of arguments"),
            }
        }
        BuiltinOp::I32Not => {
            if args.len() != 1 {
                panic!("I32Not: requires 1 argument but recieves {}", args.len())
            }
            match &args[0] {
                Value::I32(v) => Value::I32(v.reverse_bits()),
                _ => panic!("I32Not: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Add => {
            if args.len() != 2 {
                panic!("I64Add: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 + v2),
                _ => panic!("I64Add: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Sub => {
            if args.len() != 2 {
                panic!("I64Sub: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 - v2),
                _ => panic!("I64Sub: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Mul => {
            if args.len() != 2 {
                panic!("I64Mul: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 * v2),
                _ => panic!("I64Mul: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Div => {
            if args.len() != 2 {
                panic!("I64Div: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 / v2),
                _ => panic!("I64Div: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Eq => {
            if args.len() != 2 {
                panic!("I64Eq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::Bool(v1 == v2),
                _ => panic!("I64Eq: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Gt => {
            if args.len() != 2 {
                panic!("I64Gt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::Bool(v1 > v2),
                _ => panic!("I64Gt: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Geq => {
            if args.len() != 2 {
                panic!("I64Geq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::Bool(v1 >= v2),
                _ => panic!("I64Geq: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Lt => {
            if args.len() != 2 {
                panic!("I64Lt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::Bool(v1 < v2),
                _ => panic!("I64Lt: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Leq => {
            if args.len() != 2 {
                panic!("I64Leq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::Bool(v1 <= v2),
                _ => panic!("I64Leq: wrong type of arguments"),
            }
        }
        BuiltinOp::I64And => {
            if args.len() != 2 {
                panic!("I64And: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 & v2),
                _ => panic!("I64And: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Or => {
            if args.len() != 2 {
                panic!("I64Or: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 | v2),
                _ => panic!("I64Or: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Xor => {
            if args.len() != 2 {
                panic!("I64Xor: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::I64(v1), Value::I64(v2)) => Value::I64(v1 ^ v2),
                _ => panic!("I64Xor: wrong type of arguments"),
            }
        }
        BuiltinOp::I64Not => {
            if args.len() != 1 {
                panic!("I64Not: requires 1 argument but receives {}", args.len())
            }
            match &args[0] {
                Value::I64(v) => Value::I64(!v),
                _ => panic!("I64Not: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Add => {
            if args.len() != 2 {
                panic!("U32Add: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 + v2),
                _ => panic!("U32Add: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Sub => {
            if args.len() != 2 {
                panic!("U32Sub: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 - v2),
                _ => panic!("U32Sub: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Mul => {
            if args.len() != 2 {
                panic!("U32Mul: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 * v2),
                _ => panic!("U32Mul: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Div => {
            if args.len() != 2 {
                panic!("U32Div: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 / v2),
                _ => panic!("U32Div: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Eq => {
            if args.len() != 2 {
                panic!("U32Eq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::Bool(v1 == v2),
                _ => panic!("U32Eq: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Gt => {
            if args.len() != 2 {
                panic!("U32Gt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::Bool(v1 > v2),
                _ => panic!("U32Gt: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Geq => {
            if args.len() != 2 {
                panic!("U32Geq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::Bool(v1 >= v2),
                _ => panic!("U32Geq: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Lt => {
            if args.len() != 2 {
                panic!("U32Lt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::Bool(v1 < v2),
                _ => panic!("U32Lt: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Leq => {
            if args.len() != 2 {
                panic!("U32Leq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::Bool(v1 <= v2),
                _ => panic!("U32Leq: wrong type of arguments"),
            }
        }
        BuiltinOp::U32And => {
            if args.len() != 2 {
                panic!("U32And: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 & v2),
                _ => panic!("U32And: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Or => {
            if args.len() != 2 {
                panic!("U32Or: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 | v2),
                _ => panic!("U32Or: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Xor => {
            if args.len() != 2 {
                panic!("U32Xor: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U32(v1), Value::U32(v2)) => Value::U32(v1 ^ v2),
                _ => panic!("U32Xor: wrong type of arguments"),
            }
        }
        BuiltinOp::U32Not => {
            if args.len() != 1 {
                panic!("U32Not: requires 1 argument but receives {}", args.len())
            }
            match &args[0] {
                Value::U32(v) => Value::U32(!v),
                _ => panic!("U32Not: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Add => {
            if args.len() != 2 {
                panic!("U64Add: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 + v2),
                _ => panic!("U64Add: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Sub => {
            if args.len() != 2 {
                panic!("U64Sub: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 - v2),
                _ => panic!("U64Sub: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Mul => {
            if args.len() != 2 {
                panic!("U64Mul: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 * v2),
                _ => panic!("U64Mul: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Div => {
            if args.len() != 2 {
                panic!("U64Div: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 / v2),
                _ => panic!("U64Div: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Eq => {
            if args.len() != 2 {
                panic!("U64Eq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::Bool(v1 == v2),
                _ => panic!("U64Eq: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Gt => {
            if args.len() != 2 {
                panic!("U64Gt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::Bool(v1 > v2),
                _ => panic!("U64Gt: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Geq => {
            if args.len() != 2 {
                panic!("U64Geq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::Bool(v1 >= v2),
                _ => panic!("U64Geq: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Lt => {
            if args.len() != 2 {
                panic!("U64Lt: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::Bool(v1 < v2),
                _ => panic!("U64Lt: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Leq => {
            if args.len() != 2 {
                panic!("U64Leq: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::Bool(v1 <= v2),
                _ => panic!("U64Leq: wrong type of arguments"),
            }
        }
        BuiltinOp::U64And => {
            if args.len() != 2 {
                panic!("U64And: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 & v2),
                _ => panic!("U64And: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Or => {
            if args.len() != 2 {
                panic!("U64Or: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 | v2),
                _ => panic!("U64Or: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Xor => {
            if args.len() != 2 {
                panic!("U64Xor: requires 2 arguments but receives {}", args.len())
            }
            match (&args[0], &args[1]) {
                (Value::U64(v1), Value::U64(v2)) => Value::U64(v1 ^ v2),
                _ => panic!("U64Xor: wrong type of arguments"),
            }
        }
        BuiltinOp::U64Not => {
            if args.len() != 1 {
                panic!("U64Not: requires 1 argument but receives {}", args.len())
            }
            match &args[0] {
                Value::U64(v) => Value::U64(!v),
                _ => panic!("U64Not: wrong type of arguments"),
            }
        }
    }
}
