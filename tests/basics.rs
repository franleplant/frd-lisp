use frd_lisp::eval_file;

#[test]
fn basics() {
    let _ = env_logger::try_init();
    let res = eval_file("./tests/fib.flp");
    println!("{:?}", res);

    // Compare strings as a way of avoiding re creating the expected array
    let string_res = format!("{:?}", res);
    assert_eq!(
        string_res,
        "[Ok(Nill), Ok(1), Ok(1), Ok(2), Ok(3), Ok(5), Ok(8), Ok(13)]"
    );
}
