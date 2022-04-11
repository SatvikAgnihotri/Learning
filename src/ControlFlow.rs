fn main() {
    //F to C
    let mut number = 212;
    number -= 32;
    number *= 5;
    number /= 9;

    println!("In Celsius, the temperature is {}", number);

    //Fibbonaci Sequence
    let mut sequence = vec![0, 1];
    let n = 10;

    for _number in (1..n).rev() {
        let length: usize = sequence.len().try_into().unwrap();
        let last_term = length - 1;
        let second_term = length - 2;
        let a = sequence[last_term];
        let b = sequence[second_term];
        let c = a + b;
        sequence.push(c);
        println!("{:?}", sequence);
    }
}
