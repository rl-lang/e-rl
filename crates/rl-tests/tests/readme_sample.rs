use rl_embed::run;

#[test]
fn readme_sample() {
    let code = r#"
get println from std::io

fn fib(int n) {
    if (n < 2) {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

dec int total = 0
dec int i = 0
while (i <= 10) {
    total += fib(i)
    i += 1
}
println("sum of fib(0..10) = ", total)
"#;
    let (_, output) = run(code, "readme.rl", None).unwrap();
    assert_eq!(output, "sum of fib(0..10) = 143\n");
}