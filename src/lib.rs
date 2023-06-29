pub mod fib;

#[test]
fn it_works() {
  std::thread::sleep(std::time::Duration::from_secs(10));
    assert_eq!(2 + 2, 4);
}
