fn get_mass(x: u64) -> u64 {
    let mut fuel = x / 3 - 2;
    if fuel > 6 {
        fuel += get_mass(fuel);
    }
    fuel
}
fn main() {
    println!("{:?}", std::fs::read_to_string("inputs.txt").unwrap().lines().collect::<Vec<&str>>().iter().map(|input| get_mass(input.clone().parse::<u64>().unwrap())).sum::<u64>());
}
