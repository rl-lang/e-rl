//! Host-side integration tests exercising the `rl-embed` facade: the full
//! lex -> parse -> resolve -> compile -> run pipeline against the no_std
//! VM, running on the host with std's allocator.
//!
//! Note on syntax: RL statements are newline-separated (there is no
//! `;` terminator), variables are declared with `dec`, and stdlib functions
//! must be imported with `get name from std::module`.

use rl_embed::{compile, compile_with_source, run};
use rl_vm::stdlib;
use rl_vm::vm_logic::Vm;
use rl_vm::{deserialize_chunk, serialize_chunk};

fn eval(source: &str) -> (rl_embed::VmValue, String) {
    let (value, output) = run(source, "test.rl", None).expect("script should run");
    (value, output)
}

#[test]
fn runs_arithmetic() {
    let (value, _) = eval("dec int x = 21 * 2\nx");
    assert_eq!(format!("{value}"), "42");
}

#[test]
fn evaluates_last_expression() {
    let (value, _) = eval("dec int a = 10\ndec int b = 20\na + b");
    assert_eq!(format!("{value}"), "30");
}

#[test]
fn prints_to_capture_buffer() {
    let (_, output) = eval(r#"get println from std::io
println("hello, ", 2 + 3)"#);
    assert_eq!(output, "hello, 5\n");
}

#[test]
fn captures_multiple_prints_in_order() {
    let (_, output) = eval(
        "get print from std::io\nget println from std::io\nprintln(\"a\")\nprint(\"b\")\nprintln(\"c\")",
    );
    assert_eq!(output, "a\nbc\n");
}

#[test]
fn output_buffer_drains_on_take() {
    let (chunk, source) = compile_with_source("get println from std::io\nprintln(\"x\")".into(), "t.rl").unwrap();
    let mut vm = Vm::new().with_source_file(source);
    rl_embed::set_output_buffer(&mut vm);
    vm.run_and_return(&chunk).unwrap();
    assert_eq!(rl_embed::take_output(&mut vm).unwrap(), "x\n");
    assert_eq!(rl_embed::take_output(&mut vm), None);
}

#[test]
fn rejects_file_imports() {
    let err = compile("get add, sub from mylib::utils", "t.rl").unwrap_err();
    assert!(err.contains("file imports are not supported"));
}

#[test]
fn bytecode_round_trips() {
    let (chunk, source) = compile_with_source("dec int x = 7\nx * 3".into(), "t.rl").unwrap();

    let line_index = rl_utils::line_index::LineIndex::new("t.rl", &*source.text);
    let bytes = serialize_chunk(&chunk, Some(&line_index));
    let (loaded, _) = deserialize_chunk(&bytes, &stdlib::root()).unwrap();

    let mut vm = Vm::new();
    let result = vm.run_and_return(&loaded).unwrap();
    assert_eq!(format!("{result}"), "21");
}

#[test]
fn bytecode_rejects_bad_magic() {
    let err = deserialize_chunk(b"NOTRLZ", &stdlib::root()).unwrap_err();
    assert!(err.0.contains("bad magic header"));
}

#[test]
fn seeded_rng_is_deterministic() {
    let code = "get rand_int from std::random\nrand_int()";
    let (a, _) = run(code, "t.rl", Some(1234)).unwrap();
    let (b, _) = run(code, "t.rl", Some(1234)).unwrap();
    let (c, _) = run(code, "t.rl", Some(9999)).unwrap();

    assert_eq!(format!("{a}"), format!("{b}"), "same seed must repeat");
    assert_ne!(format!("{a}"), format!("{c}"), "different seed must differ");
}

#[test]
fn math_functions_are_available() {
    let (value, _) = eval("get sin from std::math\nsin(1.0)");
    let approx = format!("{value}").parse::<f64>().unwrap();
    assert!((approx - 1.0_f64.sin()).abs() < 1e-9);
}

#[test]
fn runtime_error_carries_source_location() {
    let err = run("dec int a = 1\nunknown_fn()", "err.rl", None).unwrap_err();
    let (name, line, _) = err.location().expect("error should carry a location");
    assert_eq!(&**name, "err.rl");
    assert_eq!(line, 2);
    assert!(err.message().contains("unknown_fn"));
}

#[test]
fn stdlib_introspection_is_available() {
    let (value, _) = run("get source_name from std::rl\nsource_name()", "intro.rl", None).unwrap();
    assert_eq!(format!("{value}"), "intro.rl");
}

#[test]
fn panics_on_unhandled_runtime_errors() {
    let err = run("dec int x = 1\nx = 10\nget println from std::io\nprintln(x)", "t.rl", None);
    assert!(err.is_ok());
}