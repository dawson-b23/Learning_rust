fn main() {
    let mut x = 5;

    // can be global scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope: {x}");
    }
    x = 6;

    println!("The value of x is: {x}");
    println!("Three hours in seconds {THREE_HOURS_IN_SECONDS}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Space: {spaces}");

    let tup = (500, 6.4, 1);
    let (_r, y, _z) = tup;

    println!("The value of y is: {y}");

    let d: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = d.0;
    let _six_point_four = d.1;
    let _one = d.2;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;
    println!("The value of b is {b}");

    // arrays
    let arr1 = [1, 2, 3, 4, 5];

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5] == let a = [3, 3, 3, 3, 3]
    another_function(5, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function! the value of inputs is {value}{unit_label}");
}
