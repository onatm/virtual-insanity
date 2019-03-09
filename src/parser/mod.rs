use nom::rest;

pub struct Parser;

#[derive(Debug, PartialEq)]
enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
enum Segment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Pointer,
    Static,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Arithmetic(Arithmetic),
    Push { segment: Segment, val: i32 },
    Pop { segment: Segment, val: i32 },
    // Label,
    // Goto,
    // If,
    // Function,
    // Return,
    // Call,
}

named!(parse_operation<&str, Operation>,
    alt!(
        parse_arithmetic_operation |
        parse_push_operation |
        parse_pop_operation
    )
);

named!(parse_arithmetic_operation<&str, Operation>,
    alt!(
        do_parse!(
            tag!("add") >>
            (Operation::Arithmetic(Arithmetic::Add))
        ) |
        do_parse!(
            tag!("sub") >>
            (Operation::Arithmetic(Arithmetic::Sub))
        ) |
        do_parse!(
            tag!("neg") >>
            (Operation::Arithmetic(Arithmetic::Neg))
        )|
        do_parse!(
            tag!("eq") >>
            (Operation::Arithmetic(Arithmetic::Eq))
        )|
        do_parse!(
            tag!("gt") >>
            (Operation::Arithmetic(Arithmetic::Gt))
        )|
        do_parse!(
            tag!("lt") >>
            (Operation::Arithmetic(Arithmetic::Lt))
        )|
        do_parse!(
            tag!("and") >>
            (Operation::Arithmetic(Arithmetic::And))
        )|
        do_parse!(
            tag!("or") >>
            (Operation::Arithmetic(Arithmetic::Or))
        )|
        do_parse!(
            tag!("not") >>
            (Operation::Arithmetic(Arithmetic::Not))
        )
    )
);

named!(parse_push_operation<&str, Operation>,
    do_parse!(
        tag!("push") >>
        segment: parse_segment >>
        val: rest >>
        (Operation::Push{segment: segment, val: val.parse::<i32>().unwrap() })
    )
);

named!(parse_pop_operation<&str, Operation>,
    do_parse!(
        tag!("pop") >>
        segment: parse_segment >>
        val: rest >>
        (Operation::Pop{segment: segment, val: val.parse::<i32>().unwrap() })
    )
);

named!(parse_segment<&str, Segment>,
    ws!(
        alt!(
            do_parse!(tag!("constant") >> (Segment::Constant)) |
            do_parse!(tag!("local") >> (Segment::Local)) |
            do_parse!(tag!("argument") >> (Segment::Argument)) |
            do_parse!(tag!("this") >> (Segment::This)) |
            do_parse!(tag!("that") >> (Segment::That)) |
            do_parse!(tag!("pointer") >> (Segment::Pointer)) |
            do_parse!(tag!("static") >> (Segment::Static))
        )
    )
);

#[test]
fn arithmetic_operation() {
    let expected_operation = Operation::Arithmetic(Arithmetic::Add);

    let operation = parse_arithmetic_operation("add");
    let (_, operation) = operation.unwrap();

    assert_eq!(operation, expected_operation);
}

#[test]
fn push_operation() {
    let expected_operation = Operation::Push {
        segment: Segment::This,
        val: 42,
    };

    let operation = parse_push_operation("push this 42");
    let (_, operation) = operation.unwrap();

    assert_eq!(operation, expected_operation);
}

#[test]
fn pop_operation() {
    let expected_operation = Operation::Pop {
        segment: Segment::Local,
        val: 42,
    };

    let operation = parse_pop_operation("pop local 42");
    let (_, operation) = operation.unwrap();

    assert_eq!(operation, expected_operation);
}

#[test]
fn parse_any_operation() {
    let expected_operation = Operation::Push {
        segment: Segment::Pointer,
        val: 3,
    };

    let operation = parse_operation("push pointer 3");
    let (_, operation) = operation.unwrap();

    assert_eq!(operation, expected_operation);
}
