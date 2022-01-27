use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    println!("guessing game starting");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

    println!("Please select a number from 1 - 100");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    let guess:u32 = guess.trim().parse()
                    .expect("Please input a number");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("way less"),
        Ordering::Greater => println!("Greater than the guess"),
        Ordering::Equal => {
            println!("Yay! you won");
            break;
         }
        }
    }
}