use crate::{builtins::console::formatter, Context, Value};

#[test]
fn formatter_no_args_is_empty_string() {
    let mut context = Context::new();
    assert_eq!(formatter(&[], &mut context).unwrap(), "");
}

#[test]
fn formatter_empty_format_string_is_empty_string() {
    let mut context = Context::new();
    let val = Value::string("".to_string());
    assert_eq!(formatter(&[val], &mut context).unwrap(), "");
}

#[test]
fn formatter_format_without_args_renders_verbatim() {
    let mut context = Context::new();
    let val = [Value::string("%d %s %% %f")];
    let res = formatter(&val, &mut context).unwrap();
    assert_eq!(res, "%d %s %% %f");
}

#[test]
fn formatter_empty_format_string_concatenates_rest_of_args() {
    let mut context = Context::new();

    let val = [
        Value::string(""),
        Value::string("to powinno zostać"),
        Value::string("połączone"),
    ];
    let res = formatter(&val, &mut context).unwrap();
    assert_eq!(res, " to powinno zostać połączone");
}

#[test]
fn formatter_utf_8_checks() {
    let mut context = Context::new();

    let val = [
        Value::string("Są takie chwile %dą %są tu%sów %привет%ź".to_string()),
        Value::integer(123),
        Value::rational(1.23),
        Value::string("ł".to_string()),
    ];
    let res = formatter(&val, &mut context).unwrap();
    assert_eq!(res, "Są takie chwile 123ą 1.23ą tułów %привет%ź");
}

#[test]
fn formatter_trailing_format_leader_renders() {
    let mut context = Context::new();

    let val = [
        Value::string("%%%%%".to_string()),
        Value::string("|".to_string()),
    ];
    let res = formatter(&val, &mut context).unwrap();
    assert_eq!(res, "%%% |");
}

#[test]
#[allow(clippy::approx_constant)]
fn formatter_float_format_works() {
    let mut context = Context::new();

    let val = [Value::string("%f".to_string()), Value::rational(3.1415)];
    let res = formatter(&val, &mut context).unwrap();
    assert_eq!(res, "3.141500");
}
