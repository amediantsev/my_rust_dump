// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// const FAHRENHEIT_TO_CELSIUS: f32 = 5.0 / 9.0;
//
// fn run_guess_number_game() {
//     println!("Guess the number!");
//
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//
//     loop {
//         println!("Please input your guess.");
//
//         let mut guess = String::new();
//
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         println!("You guessed: {guess}");
//
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
//
//
// fn convert_fahrenheit_to_celsius(temperature: f32) -> f32 {
//     (temperature - 32.0) * FAHRENHEIT_TO_CELSIUS
// }


fn generate_fib(n: u32) -> u32 {
    if n < 2 {
        n
    }
    else {
        generate_fib(n - 1) + generate_fib(n - 2)
    }
}


fn main() {
    // run_guess_number_game();


    // let result = convert_fahrenheit_to_celsius(115.0);
    // println!("celsius is {result}")

    let result = generate_fib(10);
    println!("result is {result}");


}
