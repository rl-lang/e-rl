use rl_embed::run;

#[test]
fn test_assert_pass() {
    let code = r#"
        get assert from std::debug
        assert(true)
        assert(1 == 1, "math works")
    "#;
    let (val, _) = run(code, "test.rl", None).unwrap();
    assert_eq!(val.to_string(), "null");
}

#[test]
fn test_assert_fail() {
    let code = r#"
        get assert from std::debug
        assert(false)
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("assertion failed"));
}

#[test]
fn test_assert_eq_pass() {
    let code = r#"
        get assert_eq from std::debug
        assert_eq(42, 42)
        assert_eq("foo", "foo", "strings match")
    "#;
    run(code, "test.rl", None).unwrap();
}

#[test]
fn test_assert_eq_fail() {
    let code = r#"
        get assert_eq from std::debug
        assert_eq(42, 43, "oops")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("oops: assert_eq failed: left `42` (int) != right `43` (int)"));
}

#[test]
fn test_assert_ne_pass() {
    let code = r#"
        get assert_ne from std::debug
        assert_ne(42, 43)
        assert_ne("foo", "bar", "strings differ")
    "#;
    run(code, "test.rl", None).unwrap();
}

#[test]
fn test_assert_ne_fail() {
    let code = r#"
        get assert_ne from std::debug
        assert_ne(42, 42, "oops")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("oops: assert_ne failed: left `42` (int) == right `42` (int)"));
}

#[test]
fn test_assert_lt_le_gt_ge_pass() {
    let code = r#"
        get assert_lt, assert_le, assert_gt, assert_ge from std::debug
        assert_lt(1, 2)
        assert_le(2, 2)
        assert_gt(3, 2)
        assert_ge(3, 3)
    "#;
    run(code, "test.rl", None).unwrap();
}

#[test]
fn test_assert_lt_fail() {
    let code = r#"
        get assert_lt from std::debug
        assert_lt(2, 1, "must be less")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("must be less: assert_lt failed: `2` vs `1`"));
}

#[test]
fn test_assert_approx_eq() {
    let code = r#"
        get assert_approx_eq from std::debug
        assert_approx_eq(1.0, 1.0000000001)
        assert_approx_eq(1.0, 1.1, 0.2)
    "#;
    run(code, "test.rl", None).unwrap();
}

#[test]
fn test_assert_approx_eq_fail() {
    let code = r#"
        get assert_approx_eq from std::debug
        assert_approx_eq(1.0, 1.1)
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("assert_approx_eq failed: `1.0` and `1.1` differ by more than 0.000000001"));
}

#[test]
fn test_panic() {
    let code = r#"
        get panic from std::debug
        panic("system failure")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("system failure"));
}

#[test]
fn test_unreachable() {
    let code = r#"
        get unreachable from std::debug
        unreachable("should not be here")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("internal error: entered unreachable code: should not be here"));
}

#[test]
fn test_todo() {
    let code = r#"
        get todo from std::debug
        todo("implement later")
    "#;
    let err = run(code, "test.rl", None).unwrap_err();
    assert!(err.message().contains("not yet implemented: implement later"));
}

#[test]
fn test_dbg() {
    let code = r#"
        get dbg from std::debug
        dec int x = dbg(42)
        x
    "#;
    let (val, output) = run(code, "test.rl", None).unwrap();
    assert_eq!(val.to_string(), "42");
    assert_eq!(output, "[dbg] 42 (int)\n");
}

#[test]
fn test_type_of() {
    let code = r#"
        get type_of from std::debug
        type_of(42)
    "#;
    let (val, _) = run(code, "test.rl", None).unwrap();
    assert_eq!(val.to_string(), "int");
}
