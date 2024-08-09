fn main() {
    println!("Guess the number!");

    let x = 5;
    println!("the value of x is {x}");

    let x = 6;

    let x = x + 1;
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("spance lenth is {spaces}");

    let x = "string";

    let space = "   ";

    x = space.len(); // 这样是不行的
                     // let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}
