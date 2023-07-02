pub fn part1(vec: &Vec<String>) {
    let mut accum: Vec<usize> = Vec::new();
    let mut count: usize = 0;

    for x in vec {
        match x.as_str() {
            "" => count += 1,
            item => {
                let cal: usize = item.parse().unwrap();
                if accum.len() == count + 1 {
                    accum[count] += cal;
                } else {
                    accum.push(cal);
                }
            }
        }
    }
    println!("[Part 1] Max value is: {}",accum.iter().max().unwrap())
}

pub fn part2(vec: &Vec<String>) {
    let mut accum: Vec<usize> = Vec::new();
    let mut count: usize = 0;

    for x in vec {
        match x.as_str() {
            "" => count += 1,
            item => {
                let cal: usize = item.parse().unwrap();
                if accum.len() == count + 1 {
                    accum[count] += cal;
                } else {
                    accum.push(cal);
                }
            }
        }
    }
    accum.sort();
    accum.reverse();

    let total = accum[0] + accum[1] + accum[2];
    println!("[Part 2] Max 3 value is: {}",total)

}
