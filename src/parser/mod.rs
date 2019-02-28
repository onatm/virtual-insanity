use nom::rest;

pub struct Parser;

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
    // Arithmetic,
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
        parse_push_operation |
        parse_pop_operation
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
