//! Scalar literals, primitive vocabulary, and operand typing.
//!
//! Derived from Jarvis peripheral-graph; operation execution belongs to consumers.

/// Supported integer types; width and signedness are language properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Integer {
    /// Unsigned eight bits.
    U8,
    /// Unsigned sixteen bits.
    U16,
    /// Unsigned thirty-two bits.
    U32,
    /// Unsigned sixty-four bits.
    U64,
    /// Signed eight bits.
    I8,
    /// Signed sixteen bits.
    I16,
    /// Signed thirty-two bits.
    I32,
    /// Signed sixty-four bits.
    I64,
}

impl Integer {
    /// Declared number of bits, independent of the host.
    #[must_use]
    pub const fn width(self) -> u32 {
        match self {
            Self::U8 | Self::I8 => 8,
            Self::U16 | Self::I16 => 16,
            Self::U32 | Self::I32 => 32,
            Self::U64 | Self::I64 => 64,
        }
    }

    /// Whether ordering and division interpret two's-complement signed values.
    #[must_use]
    pub const fn signed(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64)
    }

    const fn mask(self) -> u64 {
        u64::MAX >> (64 - self.width())
    }
}

/// A Boolean is distinct from every integer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    /// Logical truth value.
    Bool,
    /// Fixed-width bit pattern with declared interpretation.
    Integer(Integer),
}

/// A well-formed scalar. Private bits prevent out-of-range integer values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Value(Repr);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Repr {
    Bool(bool),
    Integer(Integer, u64),
}

/// Invalid scalar literal or primitive operand types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Literal is outside the declared mathematical range.
    LiteralRange,
    /// Operand types are outside the primitive's domain.
    Type,
}

impl Value {
    /// Boolean literal.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self(Repr::Bool(value))
    }

    /// Construct a literal without implicit truncation, including in static definitions.
    ///
    /// # Errors
    /// Returns `LiteralRange` if the mathematical integer does not fit.
    pub const fn integer(ty: Integer, value: i128) -> Result<Self, Error> {
        let magnitude_bits = if ty.signed() {
            ty.width() - 1
        } else {
            ty.width()
        };
        let bound = 1_i128 << magnitude_bits;
        let minimum = if ty.signed() { -bound } else { 0 };
        if value < minimum || value >= bound {
            return Err(Error::LiteralRange);
        }
        Ok(Self::bits(ty, value))
    }

    // The caller has checked the literal range; retain its declared-width bit pattern.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    const fn bits(ty: Integer, value: i128) -> Self {
        Self(Repr::Integer(ty, (value as u64) & ty.mask()))
    }

    /// Declared scalar type.
    #[must_use]
    pub const fn ty(self) -> Type {
        match self.0 {
            Repr::Bool(_) => Type::Bool,
            Repr::Integer(ty, _) => Type::Integer(ty),
        }
    }

    /// Obtain a Boolean without a truthiness conversion.
    #[must_use]
    pub const fn as_bool(self) -> Option<bool> {
        if let Repr::Bool(value) = self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// Mathematical integer value with the declared signed interpretation.
    #[must_use]
    pub fn as_integer(self) -> Option<i128> {
        let Repr::Integer(ty, bits) = self.0 else {
            return None;
        };
        let value = i128::from(bits);
        Some(if ty.signed() && bits & (1 << (ty.width() - 1)) != 0 {
            value - (1_i128 << ty.width())
        } else {
            value
        })
    }
}

/// Unary primitives and explicit conversions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unary {
    /// Boolean complement.
    Not,
    /// Integer bitwise complement.
    Complement,
    /// Widen an unsigned value, preserving signedness.
    ZeroExtend(Integer),
    /// Widen a signed value, preserving signedness.
    SignExtend(Integer),
    /// Keep low bits of a narrower type of the same signedness.
    Truncate(Integer),
    /// Change signedness at the same width.
    Reinterpret(Integer),
}

impl Unary {
    /// Check the operation domain and obtain its result type.
    ///
    /// # Errors
    /// Returns `Type` for an illegal conversion or operand.
    pub fn result_type(self, input: Type) -> Result<Type, Error> {
        if self == Self::Not {
            return (input == Type::Bool)
                .then_some(Type::Bool)
                .ok_or(Error::Type);
        }
        let Type::Integer(from) = input else {
            return Err(Error::Type);
        };
        let (to, valid) = match self {
            Self::Not => unreachable!(),
            Self::Complement => (from, true),
            Self::ZeroExtend(to) => (
                to,
                !from.signed() && !to.signed() && to.width() > from.width(),
            ),
            Self::SignExtend(to) => (
                to,
                from.signed() && to.signed() && to.width() > from.width(),
            ),
            Self::Truncate(to) => (
                to,
                from.signed() == to.signed() && to.width() < from.width(),
            ),
            Self::Reinterpret(to) => (
                to,
                from.signed() != to.signed() && to.width() == from.width(),
            ),
        };
        valid.then_some(Type::Integer(to)).ok_or(Error::Type)
    }
}

/// Strict left-to-right binary operations. Lazy Boolean operators are control
/// composition in `Expr`, not integer bitwise operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binary {
    /// Same-type equality, including Booleans.
    Equal,
    /// Same-type inequality, including Booleans.
    NotEqual,
    /// Integer ordering.
    Less,
    /// Integer ordering including equality.
    LessEqual,
    /// Integer ordering.
    Greater,
    /// Integer ordering including equality.
    GreaterEqual,
    /// Modular addition.
    Add,
    /// Modular subtraction.
    Subtract,
    /// Modular multiplication.
    Multiply,
    /// Quotient rounded toward zero at the declared width.
    /// A zero divisor yields all ones; signed minimum divided by -1 yields minimum.
    Divide,
    /// Remainder with the dividend's sign at the declared width.
    /// A zero divisor yields the dividend; signed minimum modulo -1 yields zero.
    Remainder,
    /// Bitwise conjunction.
    BitAnd,
    /// Bitwise disjunction.
    BitOr,
    /// Bitwise exclusive disjunction.
    BitXor,
    /// Left shift; unsigned count, zero for oversized counts.
    ShiftLeft,
    /// Unsigned logical right shift; zero for oversized counts.
    ShiftRight,
    /// Signed right shift; sign fill for oversized counts.
    ShiftArithmetic,
}

impl Binary {
    /// Check both operand types, with no implicit promotions.
    ///
    /// # Errors
    /// Returns `Type` for incompatible operands.
    pub fn result_type(self, left: Type, right: Type) -> Result<Type, Error> {
        match self {
            Self::Equal | Self::NotEqual if left == right => return Ok(Type::Bool),
            Self::ShiftLeft | Self::ShiftRight | Self::ShiftArithmetic => {
                let (Type::Integer(a), Type::Integer(b)) = (left, right) else {
                    return Err(Error::Type);
                };
                return (!b.signed()
                    && match self {
                        Self::ShiftRight => !a.signed(),
                        Self::ShiftArithmetic => a.signed(),
                        _ => true,
                    })
                .then_some(left)
                .ok_or(Error::Type);
            }
            _ => {}
        }
        if left != right || !matches!(left, Type::Integer(_)) {
            return Err(Error::Type);
        }
        Ok(
            if matches!(
                self,
                Self::Less | Self::LessEqual | Self::Greater | Self::GreaterEqual
            ) {
                Type::Bool
            } else {
                left
            },
        )
    }
}
