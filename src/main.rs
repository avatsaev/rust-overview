fn borrow_sum(v: &Vec<i32>) -> i32 {
  let mut sum = 0;
  for val in v  {
      sum += *val;
  }
  return sum;
}

fn cap_values_own(v: &mut Vec<i32>, max: i32) {
    for i in 0..v.len() {
        if v[i] > max {
            v[i]= max;
        }
    }
}

fn main() {
    let mut vec = vec![1,2,3,5,6,7,8];
    //   let res = borrow_sum(&vec);
    //   println!("Sum of {} values: {}", vec.len(), res );
    cap_values_own(&mut vec, 3);
    println!("{:?}", vec);

}
