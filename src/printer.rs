use crate::{ArrayType, Value};
use std::fmt::Write;

fn indent(w: &mut impl Write, indentation_level: i32) {
    for _ in 0..indentation_level {
        write!(w, " ").unwrap();
    }
}

pub fn print(w: &mut impl Write, value: &Value, indentation_level: i32) {
    match value {
        Value::Bool(b) => {
            if *b {
                write!(w, "true").unwrap();
            } else {
                write!(w, "false").unwrap();
            }
        }
        Value::Int(i) => {
            write!(w, "{i}").unwrap();
        }
        Value::Float(f) => {
            write!(w, "{f:?}").unwrap();
        }
        Value::String(s) => {
            write!(w, "{s:?}").unwrap();
        }
        Value::Array(a, array_type) => {
            if *array_type == ArrayType::List {
                write!(w, "( ").unwrap();
            } else {
                write!(w, "[ ").unwrap();
            }
            for (i, v) in a.iter().enumerate() {
                print(w, v, indentation_level + 4);
                if i != a.len() - 1 {
                    write!(w, ", ").unwrap();
                }
            }
            if *array_type == ArrayType::List {
                write!(w, " )").unwrap();
            } else {
                write!(w, " ]").unwrap();
            }
        }
        Value::Object(o) => {
            writeln!(w, "{{").unwrap();
            for (name, v) in o {
                indent(w, indentation_level);
                write!(w, "{name} : ").unwrap();
                print(w, v, indentation_level + 4);
                writeln!(w, ";").unwrap();
            }
            indent(w, indentation_level - 4);
            write!(w, "}}").unwrap();
        }
    }
}
