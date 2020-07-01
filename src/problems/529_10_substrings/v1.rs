// https://projecteuler.net/problem=529
fn checks(n: i64) -> bool {
    let value_of_sum : i64 = 10;
    let mut n_string = n.to_string();
    

    let mut start = 1;
    let mut sum  = n_string[0..1].parse::<i64>().unwrap();
    for i in 1..=n_string.len() {
        println!("{}", n_string[0..1].parse::<i64>().unwrap());
        sum += n_string[i-1..i].parse::<i64>().unwrap();
        println!(">, {}", sum);
        while sum > 10 {
            sum -= n_string[start-1..start].parse::<i64>().unwrap();
            
            start+=1;
        }
        if sum == 10 {
            println!("!, {}", sum);
            return true;
        }
    }


    return false;
}

pub fn run() {
    let mut _triangle_number = 45;
    println!("{}", checks(28546));
    // println!("Triangle number: {}", triangle_number);
}