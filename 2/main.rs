use std::fs;

fn main() {
  let contents: String = fs::read_to_string("input.txt").unwrap();
  let lines: Vec<&str> = contents.lines().collect();
  let actions: Vec<Action> = lines.iter().map(|x| str_to_action(x)).collect();

  let mut horizontal = 0;
  let mut depth = 0;
  let mut aim = 0;

  for action in actions {
    match action.direction.as_str() {
      "up" => {
        // depth = depth - action.value;
        aim = aim - action.value;
      },
      "down" => {
        // depth = depth + action.value;
        aim = aim + action.value;
      },
      "forward" => {
        horizontal = horizontal + action.value;
        depth = depth + (aim * action.value);
      }
      _ => println!("pls don't hit this")
    }
  }

  println!("{}", horizontal * depth)
}

fn str_to_action(line: &str) -> Action {
  let strings: Vec<&str> = line.split(" ").collect();

  return Action {
    direction: strings.get(0).unwrap().to_string(),
    value: strings.get(1).unwrap().parse::<i32>().unwrap()
  };
}

struct Action {
  direction: String,
  value: i32,
}
