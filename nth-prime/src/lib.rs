pub fn nth(n: u32) -> u32 {
    let mut prime_numbers : Vec<u32> = Vec::new();
    let mut current_number = 2;
    loop {
        let mut current_lower_number = 2;
        let mut prime = true;
        while current_lower_number < current_number {
            if current_number % current_lower_number == 0 {
                prime = false;
                break;
            }
            current_lower_number += 1;
        }

        if prime {
            prime_numbers.push(current_number);
        }
        if prime_numbers.len() as u32 == n + 1  {
            return *prime_numbers.get(n as usize).unwrap();
        }
        current_number+=1;
    }
}