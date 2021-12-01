use std::fs;

fn main() {
  let contents = fs::read_to_string("part_1.txt").expect("Something went wrong reading the file");
  let mut splits = contents.lines().map(|x| x.parse::<i32>().unwrap());

  let mut increase_count = 0;

  let mut a1 = splits.nth(0).unwrap();
  let mut a2 = splits.nth(0).unwrap();
  let mut a3 = splits.nth(0).unwrap();

  for s in splits {
    let b1 = a2;
    let b2 = a3;
    let b3 = s;

    let a_sum = a1 + a2 + a3;
    let b_sum = b1 + b2 + b3;

    if a_sum < b_sum {
      increase_count = increase_count + 1;
    }

    a1 = a2;
    a2 = a3;
    a3 = b3;
  }

  println!("{}", increase_count);
}
