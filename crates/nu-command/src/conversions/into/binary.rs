use nu_protocol::{
    ast::Call,
    engine::{Command, EvaluationContext},
    Example, ShellError, Signature, Span, SyntaxShape, Value,
};

pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "into binary"
    }

    fn signature(&self) -> Signature {
        Signature::build("into binary").rest(
            "rest",
            SyntaxShape::CellPath,
            "column paths to convert to binary (for table input)",
        )
    }

    fn usage(&self) -> &str {
        "Convert value to a binary primitive"
    }

    fn run(
        &self,
        context: &EvaluationContext,
        call: &Call,
        input: Value,
    ) -> Result<nu_protocol::Value, nu_protocol::ShellError> {
        into_binary(context, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert string to a nushell binary primitive",
                example: "'This is a string that is exactly 52 characters long.' | into binary",
                result: Some(Value::Binary {
                    val: "This is a string that is exactly 52 characters long."
                        .to_string()
                        .as_bytes()
                        .to_vec(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a number to a nushell binary primitive",
                example: "1 | into binary",
                result: Some(Value::Binary {
                    val: i64::from(1).to_le_bytes().to_vec(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a boolean to a nushell binary primitive",
                example: "$true | into binary",
                result: Some(Value::Binary {
                    val: i64::from(1).to_le_bytes().to_vec(),
                    span: Span::unknown(),
                }),
            },
            Example {
                description: "convert a filesize to a nushell binary primitive",
                example: "ls | where name == LICENSE | get size | into binary",
                result: None,
            },
            Example {
                description: "convert a filepath to a nushell binary primitive",
                example: "ls | where name == LICENSE | get name | path expand | into binary",
                result: None,
            },
            Example {
                description: "convert a decimal to a nushell binary primitive",
                example: "1.234 | into binary",
                result: Some(Value::Binary {
                    val: 1.234f64.to_le_bytes().to_vec(),
                    span: Span::unknown(),
                }),
            },
        ]
    }
}

fn into_binary(
    _context: &EvaluationContext,
    call: &Call,
    input: Value,
) -> Result<nu_protocol::Value, nu_protocol::ShellError> {
    let head = call.head;
    // let column_paths: Vec<CellPath> = call.rest(context, 0)?;

    input.map(head, move |v| {
        action(v, head)
        // FIXME: Add back in column path support
        // if column_paths.is_empty() {
        //     action(v, head)
        // } else {
        //     let mut ret = v;
        //     for path in &column_paths {
        //         ret =
        //             ret.swap_data_by_cell_path(path, Box::new(move |old| action(old, old.tag())))?;
        //     }

        //     Ok(ret)
        // }
    })
}

fn int_to_endian(n: i64) -> Vec<u8> {
    if cfg!(target_endian = "little") {
        n.to_le_bytes().to_vec()
    } else {
        n.to_be_bytes().to_vec()
    }
}

fn float_to_endian(n: f64) -> Vec<u8> {
    if cfg!(target_endian = "little") {
        n.to_le_bytes().to_vec()
    } else {
        n.to_be_bytes().to_vec()
    }
}

pub fn action(input: Value, span: Span) -> Value {
    match input {
        Value::Binary { .. } => input,
        Value::Int { val, .. } => Value::Binary {
            val: int_to_endian(val),
            span,
        },
        Value::Float { val, .. } => Value::Binary {
            val: float_to_endian(val),
            span,
        },
        Value::Filesize { val, .. } => Value::Binary {
            val: int_to_endian(val),
            span,
        },
        Value::String { val, .. } => Value::Binary {
            val: val.as_bytes().to_vec(),
            span,
        },
        Value::Bool { val, .. } => Value::Binary {
            val: int_to_endian(if val { 1i64 } else { 0 }),
            span,
        },
        Value::Date { val, .. } => Value::Binary {
            val: val.format("%c").to_string().as_bytes().to_vec(),
            span,
        },

        _ => Value::Error {
            error: ShellError::UnsupportedInput("'into binary' for unsupported type".into(), span),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}