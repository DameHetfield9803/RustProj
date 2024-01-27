// adds n numbers together where x is the values to be added
pub fn recursive_sum(x: i32) -> i32 {
    if x <= 0 {
        return x;
    }
    return x + recursive_sum(x - 1);
}

pub fn bubble_sort(x: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..x.len() {
            if x[i - 1] > x[i] {
                x.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

pub fn main() {
    // recursive sum executing based on users input
    let mut z = String::new();
    println!("Enter an integer to execute the recursive sum function: ");
    let bytes = std::io::stdin().read_line(&mut z).unwrap();
    println!(
        "The sum of the numbers from 1 to {} is {}",
        z.trim(),
        recursive_sum(z.trim().parse::<i32>().unwrap())
    );
    println!("Number of bytes took : {}", bytes);

    // bubble sort executing based on users input
    let mut y = String::new();
    println!("Enter a list of numbers to execute the bubble sort function: ");
    let bytes = std::io::stdin().read_line(&mut y).unwrap();
    let mut y: Vec<i32> = y.split_whitespace().map(|s| s.parse().unwrap()).collect();
    bubble_sort(&mut y);
    println!("The sorted list is {:?}", y);
    println!("Number of bytes took : {}", bytes);
}
