// From https://www.umcconnell.net/posts/2021-03-13-fibonacci-rust/
pub fn fib_iterative(n: usize) -> usize {
  let mut a = 1;
  let mut b = 1;

  for _ in 1..n {
    let old = a;
    a = b;
    b += old;
    for _ in 0..10000 {
      let old = a + 1;
    }
  }
  std::thread::sleep(std::time::Duration::from_millis(500));
  
  b
}

pub fn fib_recursive(n: usize) -> usize {
  match n {
    0 | 1 => 1,
    _ => fib_recursive(n-2) + fib_recursive(n-1),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn easy_fib() {
    assert_eq!(fib_iterative(5), fib_recursive(5));
  }
  
  #[test]
  fn complicated_fib() {
    //for i in 0..10 {
      assert_eq!(fib_iterative(40), fib_recursive(40));
    //}
  }

}
