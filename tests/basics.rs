
use frd_lisp::eval_file;

#[test]
fn basics() {
    let _ = env_logger::try_init();
    let res = eval_file("./tests/fib.flp");
    println!("{:?}", res);

    // Compare strings as a way of avoiding re creating the expected array
    let string_res = format!("{:?}", res);
    assert_eq!(string_res, "[Nill, 1, 1, 2, 3, 5, 8, 13]");
}
