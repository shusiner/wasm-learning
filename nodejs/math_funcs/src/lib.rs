use wasm_bindgen::prelude::*;

// Solve a quadratic equation https://www.mathsisfun.com/quadratic-equation-solver.html
#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let ps: (f32, f32, f32) = serde_json::from_str(&params).unwrap();
  let discriminant: f32 = (ps.1 * ps.1) - (4. * ps.0 * ps.2);
  let mut solution: (f32, f32) = (0., 0.);
  if discriminant >= 0. {
    solution.0 = (((-1.) * ps.1) + discriminant.sqrt()) / (2. * ps.0);
    solution.1 = (((-1.) * ps.1) - discriminant.sqrt()) / (2. * ps.0);
    return serde_json::to_string(&solution).unwrap();
  } else {
    return String::from("not real numbers");
  }
}

#[wasm_bindgen]
pub fn t_serde_json(params: &str) -> String {
  let n1: (i64,)= serde_json::from_str(&params).unwrap();
  let n: i64 = n1.0;

  return serde_json::to_string(&n).unwrap();
}

#[wasm_bindgen]
pub fn fib_rec(params: &str) -> String {
  let n1: (i64,)= serde_json::from_str(&params).unwrap();
  let n: i64 = n1.0;

  return serde_json::to_string(&fib_rec1(n)).unwrap();
}

#[wasm_bindgen]
pub fn fib_dp(params: &str) -> String {
  let n1: (i64,)= serde_json::from_str(&params).unwrap();
  let n: i64 = n1.0;

  return serde_json::to_string(&fib_dp1(n)).unwrap();
}

#[wasm_bindgen]
pub fn fib_dp_f(params: &str) -> String {
  let n1: (i64,)= serde_json::from_str(&params).unwrap();
  let n: i64 = n1.0;

  return serde_json::to_string(&fib_dp2(n)).unwrap();
}

fn fib_rec1(n: i64) -> i64 {
  match n {
      0 => 1,
      1 => 1,
      _ => fib_rec1(n - 1) + fib_rec1(n - 2),
  }
}

fn fib_dp1(n: i64) -> i64 {
  if n==1 || n==2 {
      return 1;
  }
  let mut t1 = 1;
  let mut t2 = 1;
  for _ in 1..n-1 {
      let t3 = t2;
      t2 = t1+t2;
      t1 = t3
  }
  return t2;
}

fn fib_dp2(n: i64) -> f64 {
  if n==1 || n==2 {
      return 1.0;
  }
  let mut t1 = 1.0;
  let mut t2 = 1.0;
  for _ in 1..n-1 {
      let t3 = t2;
      t2 = t1+t2;
      t1 = t3
  }
  return t2;
}

#[wasm_bindgen]
pub fn ap1_d(params: &str) -> String {
  let ps: (i64, i64, i64) = serde_json::from_str(&params).unwrap();
  return serde_json::to_string(&ap1(ps.0,ps.1,ps.2)).unwrap();
}
#[wasm_bindgen]
pub fn ap2_d(params: &str) -> String {
  let ps: (i64, i64, i64) = serde_json::from_str(&params).unwrap();
  return serde_json::to_string(&ap2(ps.0,ps.1,ps.2)).unwrap();
}
#[wasm_bindgen]
pub fn ap3_d(params: &str) -> String {
  let ps: (i64, i64, i64) = serde_json::from_str(&params).unwrap();
  return serde_json::to_string(&ap3(ps.0,ps.1,ps.2)).unwrap();
}
#[wasm_bindgen]
pub fn ap4_d(params: &str) -> String {
  let ps: (i64, i64, i64) = serde_json::from_str(&params).unwrap();
  return serde_json::to_string(&ap4(ps.0,ps.1,ps.2)).unwrap();
}

#[wasm_bindgen]
pub fn ap1_sum_d(params: &str) -> String {
  let ps: (i64, i64, i64) = serde_json::from_str(&params).unwrap();
  return serde_json::to_string(&ap1_sum(ps.0,ps.1,ps.2)).unwrap();
}

fn ap1(a:i64,n: i64,d: i64) -> i64 { a+(n-1)*d }
fn ap2(a:i64,n: i64,t:i64) -> i64 { (t-a)/(n-1) }
fn ap3(a:i64,d: i64,t:i64) -> i64 { (t-a)/d+1 }
fn ap4(n: i64,d: i64,t:i64) -> i64 { t-(n-1)*d }
fn ap1_sum(a:i64,n: i64,d: i64) -> i64 { n*(2*a+n-1)*d/2 }

#[wasm_bindgen]
pub fn gp(params: &str) -> String {
  return serde_json::to_string(&params).unwrap();
}

#[wasm_bindgen]
pub fn is_prime(n: i32) -> String {
  return serde_json::to_string(&n).unwrap();
}