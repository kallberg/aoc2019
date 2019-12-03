use std::io::BufRead;

pub fn run(handle2: &mut BufRead) -> u64 {
  let mut sum = 0;

  for line in handle2.lines() {
    let content = line.unwrap();
    let number = content.parse::<u64>().unwrap();
    sum += compensated_fuel(number);
  }

  return sum;
}

pub fn fuel(mass: u64) -> u64 {
  let divided = mass / 3;
  if divided > 2 {
    divided - 2
  } else {
    0
  }
}

pub fn compensated_fuel(mass: u64) -> u64 {
  let fuel = fuel(mass);
  if fuel > 0 {
    fuel + compensated_fuel(fuel)
  } else {
    fuel
  }
}
