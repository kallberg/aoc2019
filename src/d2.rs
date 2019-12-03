use std::io::BufRead;

fn run_program(memory: &[i32]) -> Result<Vec<i32>, &str> {
  let mut output: Vec<i32> = memory.to_owned();
  for chunk in memory.chunks_exact(4) {
    let op = chunk[0];
    let p1 = chunk[1] as usize;
    let p2 = chunk[2] as usize;
    let p3 = chunk[3] as usize;
    if op == 1 {
      let a = output[p1];
      let b = output[p2];
      let c = a + b;
      println!("EXEC {}->{} + {}->{} = {}->{}", p1, a, p2, b, p3, c);
      output[p3] = c;
    } else if op == 2 {
      let a = output[p1];
      let b = output[p2];
      let c = a * b;
      println!("EXEC {}->{} * {}->{} = {}->{}", p1, a, p2, b, p3, c);
      output[p3] = c;
    } else if op == 99 {
      return Ok(output);
    } else {
      return Err("Bad opcode");
    }
  }
  return Ok(output);
}

pub fn print_memory(memory: &[i32]) {
  for chunk in memory.chunks(4) {
    println!("MEMORY {:?}", chunk);
  }
}

pub fn run(handle2: &mut BufRead) {
  for line in handle2.lines() {
    let mut memory: Vec<i32> = Vec::new();
    let content: String = line.unwrap();
    let ops = content.split(',');
    for op in ops {
      memory.push(op.parse::<i32>().unwrap());
    }
    let mem = run_program(&mut memory.to_owned()).unwrap();
    print_memory(&mem);
  }
}
