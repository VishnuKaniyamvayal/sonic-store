use crate::resp::{decode, RespType};

#[test]
fn test_simple_string() {
    let input = b"+OK\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::SimpleString { data, delta } => {
            assert_eq!(data, "OK");
            assert_eq!(delta, 5); // "+OK\r\n" = 5 bytes
        }
        _ => panic!("Expected SimpleString"),
    }
}

#[test]
fn test_error() {
    let input = b"-ERR unknown command\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Error { message, delta } => {
            assert_eq!(message, "ERR unknown command");
            assert_eq!(delta, 22); // "-ERR unknown command\r\n"
        }
        _ => panic!("Expected Error"),
    }
}

#[test]
fn test_integer_positive() {
    let input = b":42\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Integer { data, delta } => {
            assert_eq!(data, 42);
            assert_eq!(delta, 5); // ":42\r\n"
        }
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_integer_zero() {
    let input = b":0\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Integer { data, delta } => {
            assert_eq!(data, 0);
            assert_eq!(delta, 4); // ":0\r\n"
        }
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_integer_negative() {
    let input = b":-123\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Integer { data, delta } => {
            assert_eq!(data, -123);
            assert_eq!(delta, 7); // ":-123\r\n"
        }
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_bulk_string() {
    let input = b"$5\r\nhello\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::BulkString { data, delta } => {
            assert_eq!(data, b"hello");
            assert_eq!(delta, 11); // "$5\r\nhello\r\n" = 11 bytes
        }
        _ => panic!("Expected BulkString"),
    }
}

#[test]
fn test_bulk_string_empty() {
    let input = b"$0\r\n\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::BulkString { data, delta } => {
            assert_eq!(data, b"");
            assert_eq!(delta, 6); // "$0\r\n\r\n" = 6 bytes
        }
        _ => panic!("Expected BulkString"),
    }
}

#[test]
fn test_array_simple() {
    // *2\r\n+hello\r\n+world\r\n
    let input = b"*2\r\n+hello\r\n+world\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Array { data, .. } => {
            assert_eq!(data.len(), 2);
            match &data[0] {
                RespType::SimpleString { data, .. } => assert_eq!(data, "hello"),
                _ => panic!("Expected SimpleString in array[0]"),
            }
            match &data[1] {
                RespType::SimpleString { data, .. } => assert_eq!(data, "world"),
                _ => panic!("Expected SimpleString in array[1]"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_array_mixed() {
    // *3\r\n:1\r\n:2\r\n$3\r\nfoo\r\n
    let input = b"*3\r\n:1\r\n:2\r\n$3\r\nfoo\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Array { data, .. } => {
            assert_eq!(data.len(), 3);
            match &data[0] {
                RespType::Integer { data, .. } => assert_eq!(*data, 1),
                _ => panic!("Expected Integer in array[0]"),
            }
            match &data[1] {
                RespType::Integer { data, .. } => assert_eq!(*data, 2),
                _ => panic!("Expected Integer in array[1]"),
            }
            match &data[2] {
                RespType::BulkString { data, .. } => assert_eq!(data, b"foo"),
                _ => panic!("Expected BulkString in array[2]"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_array_nested_with_delta() {
    // *2\r\n*2\r\n+ok\r\n:7\r\n$3\r\nbar\r\n
    let input = b"*2\r\n*2\r\n+ok\r\n:7\r\n$3\r\nbar\r\n";
    let result = decode(input).unwrap();
    match result {
        RespType::Array { data, delta } => {
            assert_eq!(data.len(), 2);
            assert_eq!(delta, input.len());

            match &data[0] {
                RespType::Array { data, .. } => {
                    assert_eq!(data.len(), 2);
                    match &data[0] {
                        RespType::SimpleString { data, .. } => assert_eq!(data, "ok"),
                        _ => panic!("Expected SimpleString in nested array[0]"),
                    }
                    match &data[1] {
                        RespType::Integer { data, .. } => assert_eq!(*data, 7),
                        _ => panic!("Expected Integer in nested array[1]"),
                    }
                }
                _ => panic!("Expected nested Array in array[0]"),
            }

            match &data[1] {
                RespType::BulkString { data, .. } => assert_eq!(data, b"bar"),
                _ => panic!("Expected BulkString in array[1]"),
            }
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_empty_input() {
    let input = b"";
    let result = decode(input);
    assert!(result.is_err());
}

#[test]
fn test_invalid_prefix() {
    let input = b"!garbage\r\n";
    let result = decode(input);
    assert!(result.is_err());
}
