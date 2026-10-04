//! Literal domains and primitive typing preserved from the source IR.

use lattice_ir::{Binary, Error, Integer, Type, Unary, Value};

#[test]
fn literal_boundaries_preserve_width_signedness_and_exact_value() {
    const MAX_LEVEL: Value = match Value::integer(Integer::U8, 255) {
        Ok(value) => value,
        Err(_) => panic!("invalid static literal"),
    };
    assert_eq!(MAX_LEVEL.as_integer(), Some(255));

    let cases = [
        (Integer::U8, 8, false, 0, i128::from(u8::MAX)),
        (Integer::U16, 16, false, 0, i128::from(u16::MAX)),
        (Integer::U32, 32, false, 0, i128::from(u32::MAX)),
        (Integer::U64, 64, false, 0, i128::from(u64::MAX)),
        (
            Integer::I8,
            8,
            true,
            i128::from(i8::MIN),
            i128::from(i8::MAX),
        ),
        (
            Integer::I16,
            16,
            true,
            i128::from(i16::MIN),
            i128::from(i16::MAX),
        ),
        (
            Integer::I32,
            32,
            true,
            i128::from(i32::MIN),
            i128::from(i32::MAX),
        ),
        (
            Integer::I64,
            64,
            true,
            i128::from(i64::MIN),
            i128::from(i64::MAX),
        ),
    ];
    for (integer, width, signed, min, max) in cases {
        assert_eq!(integer.width(), width);
        assert_eq!(integer.signed(), signed);
        for n in [min, 0, max] {
            let literal = Value::integer(integer, n).unwrap();
            assert_eq!(literal.ty(), Type::Integer(integer));
            assert_eq!(literal.as_integer(), Some(n));
            assert_eq!(literal.as_bool(), None);
        }
        for n in [min - 1, max + 1] {
            assert_eq!(Value::integer(integer, n), Err(Error::LiteralRange));
        }
    }
    for b in [false, true] {
        let literal = Value::boolean(b);
        assert_eq!(literal.ty(), Type::Bool);
        assert_eq!(literal.as_bool(), Some(b));
        assert_eq!(literal.as_integer(), None);
    }
}

#[test]
fn primitive_domains_preserve_explicit_conversions_and_shift_rules() {
    use Integer::{I8, I16, U8, U16, U64};
    use Type::{Bool, Integer as Int};
    let unary = [
        (Unary::Not, Bool, Ok(Bool)),
        (Unary::Not, Int(U8), Err(Error::Type)),
        (Unary::Complement, Int(I8), Ok(Int(I8))),
        (Unary::Complement, Bool, Err(Error::Type)),
        (Unary::ZeroExtend(U16), Int(U8), Ok(Int(U16))),
        (Unary::ZeroExtend(U16), Int(I8), Err(Error::Type)),
        (Unary::ZeroExtend(U8), Int(U16), Err(Error::Type)),
        (Unary::SignExtend(I16), Int(I8), Ok(Int(I16))),
        (Unary::SignExtend(I16), Int(U8), Err(Error::Type)),
        (Unary::Truncate(U8), Int(U16), Ok(Int(U8))),
        (Unary::Truncate(U8), Int(I16), Err(Error::Type)),
        (Unary::Truncate(U16), Int(U8), Err(Error::Type)),
        (Unary::Reinterpret(I8), Int(U8), Ok(Int(I8))),
        (Unary::Reinterpret(I16), Int(U8), Err(Error::Type)),
        (Unary::Reinterpret(U8), Int(U8), Err(Error::Type)),
    ];
    for (op, input, result) in unary {
        assert_eq!(op.result_type(input), result, "{op:?} {input:?}");
    }

    let binary = [
        (Binary::Equal, Bool, Bool, Ok(Bool)),
        (Binary::Equal, Bool, Int(U8), Err(Error::Type)),
        (Binary::NotEqual, Int(U8), Int(U8), Ok(Bool)),
        (Binary::Less, Int(I8), Int(I8), Ok(Bool)),
        (Binary::Less, Bool, Bool, Err(Error::Type)),
        (Binary::Add, Int(U8), Int(U8), Ok(Int(U8))),
        (Binary::Add, Int(U8), Int(U16), Err(Error::Type)),
        (Binary::Subtract, Int(U8), Int(I8), Err(Error::Type)),
        (Binary::Multiply, Bool, Bool, Err(Error::Type)),
        (Binary::Divide, Int(I8), Int(I8), Ok(Int(I8))),
        (Binary::Remainder, Int(U16), Int(U16), Ok(Int(U16))),
        (Binary::BitAnd, Bool, Bool, Err(Error::Type)),
        (Binary::ShiftLeft, Int(I8), Int(U64), Ok(Int(I8))),
        (Binary::ShiftLeft, Int(U8), Int(I8), Err(Error::Type)),
        (Binary::ShiftRight, Int(U8), Int(U64), Ok(Int(U8))),
        (Binary::ShiftRight, Int(I8), Int(U8), Err(Error::Type)),
        (Binary::ShiftArithmetic, Int(I8), Int(U64), Ok(Int(I8))),
        (Binary::ShiftArithmetic, Int(U8), Int(U8), Err(Error::Type)),
    ];
    for (op, left, right, result) in binary {
        assert_eq!(
            op.result_type(left, right),
            result,
            "{op:?} {left:?} {right:?}"
        );
    }
}
