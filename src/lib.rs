//! The goal of this crate is a non-magical integer parser.
//!
//! It will handle hex, octal, and decimal strings of value
#![allow(dead_code)]

extern crate nom;
use nom::{IResult,ErrorKind,Needed,is_hex_digit,is_oct_digit};
use std::str;

#[inline(always)]
fn is_bool_digit(x: u8) -> bool {
    x == 49u8 || x == 48u8
}
macro_rules! gen {
    (@MULTIPARSER Starts: $prelude: expr;  Radix: $radix: expr; Identifier: $check: ident; Deligates:{$( { Name: $name: ident; Type: $out: ident; MaxChars: $limit: expr;}),*}) => {
        $(
            gen!(@NORM $name, $prelude, $check, $radix, $limit, $out);
        )*
    };
    (@NORM $name: ident, $prelude: expr, $check: ident, $radix: expr, $limit: expr, $out: ident) => {
        #[inline(always)]
        pub fn $name<'a>(i: &'a [u8]) -> IResult<&'a [u8],$out> {
            const PRELUDE: &'static [u8;2] = $prelude;
            if i.len() < 3 {
                return IResult::Incomplete(Needed::Size(3));
            }
            let temp = &i[0..2];
            if temp != PRELUDE {
                return IResult::Error(ErrorKind::Tag);
            }
            let mut index = 2usize;
            while $check(i[index]) {
                index += 1;
                if i.len() == index {
                    return IResult::Error(ErrorKind::Eof);
                }
                if index == ($limit + 2) {
                    break;
                }
            }
            /*
             * This method is prefectly safe
             * I am literally checking that
             * the values are ASCII/UTF8 encoded
             */
            let s = unsafe { str::from_utf8_unchecked(&i[2..index]) };
            IResult::Done(&i[index..], $out::from_str_radix(s,$radix).unwrap())
        }
    };
}
gen!{@MULTIPARSER
    Starts: b"0x";
    Radix: 16;
    Identifier: is_hex_digit;
    Deligates: {
        {
            Name: parse_hex_u64;
            Type: u64;
            MaxChars: 16;
        },
        {
            Name: parse_hex_u32;
            Type: u32;
            MaxChars: 8;
        },
        {
            Name: parse_hex_u16;
            Type: u16;
            MaxChars: 4;
        },
        {
            Name: parse_hex_u8;
            Type: u8;
            MaxChars: 2;
        },
        {
            Name: parse_hex_i64;
            Type: i64;
            MaxChars: 16;
        },
        {
            Name: parse_hex_i32;
            Type: i32;
            MaxChars: 8;
        },
        {
            Name: parse_hex_i16;
            Type: i16;
            MaxChars: 4;
        },
        {
            Name: parse_hex_i8;
            Type: i8;
            MaxChars: 2;
        }
    }
}
gen!{@MULTIPARSER
    Starts: b"0o";
    Radix: 8;
    Identifier: is_oct_digit;
    Deligates: {
        {
            Name: parse_oct_u64;
            Type: u64;
            MaxChars: 21;
        },
        {
            Name: parse_oct_u36;
            Type: u64;
            MaxChars: 12;
        },
        {
            Name: parse_oct_u32;
            Type: u32;
            MaxChars: 10;
        },
        {
            Name: parse_oct_u24;
            Type: u32;
            MaxChars: 8;
        },
        {
            Name: parse_oct_u16;
            Type: u16;
            MaxChars: 5;
        },
        {
            Name: parse_hex_u12;
            Type: u16;
            MaxChars: 3;
        },
        {
            Name: parse_oct_u8;
            Type: u8;
            MaxChars: 2;
        },
        {
            Name: parse_oct_i64;
            Type: i64;
            MaxChars: 21;
        },
        {
            Name: parse_oct_i36;
            Type: i64;
            MaxChars: 12;
        },
        {
            Name: parse_oct_i32;
            Type: i32;
            MaxChars: 10;
        },
        {
            Name: parse_oct_i24;
            Type: i32;
            MaxChars: 8;
        },
        {
            Name: parse_oct_i16;
            Type: i16;
            MaxChars: 5;
        },
        {
            Name: parse_hex_i12;
            Type: i16;
            MaxChars: 3;
        },
        {
            Name: parse_oct_i8;
            Type: i8;
            MaxChars: 2;
        }
    }
}
gen!{@MULTIPARSER
    Starts: b"0b";
    Radix: 2;
    Identifier: is_bool_digit;
    Deligates: {
        {
            Name: parse_bool_u64;
            Type: u64;
            MaxChars: 64;
        },
        {
            Name: parse_bool_u32;
            Type: u32;
            MaxChars: 32;
        },
        {
            Name: parse_bool_u16;
            Type: u16;
            MaxChars: 16;
        },
        {
            Name: parse_bool_u8;
            Type: u8;
            MaxChars: 8;
        },
        {
            Name: parse_bool_i64;
            Type: i64;
            MaxChars: 64;
        },
        {
            Name: parse_bool_i32;
            Type: i32;
            MaxChars: 32;
        },
        {
            Name: parse_bool_i16;
            Type: i16;
            MaxChars: 16;
        },
        {
            Name: parse_bool_i8;
            Type: i8;
            MaxChars: 8;
        }
    }
}

#[test]
fn test_parse_hex_u64() {

    //test semi normal
    let a = b"0xdeadbeef ";
    let (b,c) = match parse_hex_u64(a) {
        IResult::Done(y,x) => (y,x),
        IResult::Error(e) => panic!("Error parsing 0xdeadbeef {:?}", e),
        IResult::Incomplete(e) => panic!("Error parsing 0xdeadbeef {:?}", e),
    };
    assert_eq!(b, b" ");
    assert_eq!(c, 3735928559u64);

    //test max value
    let a = b"0xFFFFFFFFFFFFFFFF ";
    let (b,c) = match parse_hex_u64(a) {
        IResult::Done(y,x) => (y,x),
        IResult::Error(e) => panic!("Error parsing 0xFFFFFFFFFFFFFFFF {:?}", e),
        IResult::Incomplete(e) => panic!("Error parsing 0xFFFFFFFFFFFFFFFF {:?}", e),
    };
    assert_eq!(b, b" ");
    assert_eq!(c, ::std::u64::MAX);

    //test min value
    let a = b"0x0000000000000000 ";
    let (b,c) = match parse_hex_u64(a) {
        IResult::Done(y,x) => (y,x),
        IResult::Error(e) => panic!("Error parsing 0x0000000000000000 {:?}", e),
        IResult::Incomplete(e) => panic!("Error parsing 0x0000000000000000 {:?}", e),
    };
    assert_eq!(b, b" ");
    assert_eq!(c, 0u64);

    //test very short
    let a = b"0x0 ";
    let (b,c) = match parse_hex_u64(a) {
        IResult::Done(y,x) => (y,x),
        IResult::Error(e) => panic!("Error parsing 0x0 {:?}", e),
        IResult::Incomplete(e) => panic!("Error parsing 0x0 {:?}", e),
    };
    assert_eq!(b, b" ");
    assert_eq!(c, 0u64);

    //ensure prelude check works
    let a = b"0b000 ";
    match parse_hex_u64(a) {
        IResult::Done(_,_) => panic!("Test should return an error"),
        IResult::Error(ErrorKind::Tag) => { },
        IResult::Incomplete(e) => panic!("Error parsing 0xdeadbeef {:?}", e),
        IResult::Error(e) => panic!("Incorrect error type returned {:?}", e)
    };
    let a = b"1x000 ";
    match parse_hex_u64(a) {
        IResult::Done(_,_) => panic!("Test should return an error"),
        IResult::Error(ErrorKind::Tag) => { },
        IResult::Incomplete(e) => panic!("Error parsing 0xdeadbeef {:?}", e),
        IResult::Error(e) => panic!("Incorrect error type returned {:?}", e)
    };

    //ensure run on values terminate properly
    let a = b"0xFFFFFFFFFFFFFFFFFFFF ";
    let (b,c) = match parse_hex_u64(a) {
        IResult::Done(y,x) => (y,x),
        IResult::Error(e) => panic!("Error runon {:?}", e),
        IResult::Incomplete(e) => panic!("Error parsing ruon {:?}", e),
    };
    assert_eq!(b, b"FFFF ");
    assert_eq!(c, ::std::u64::MAX);
}
