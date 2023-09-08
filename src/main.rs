use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

//    println!("Секретное число :{secret_number}");
    
    loop {
        println!("Введите свое число:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {guess}\n"); //коментарий

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ваше число меньше!\n"),
            Ordering::Greater => println!("Ваше число больше!\n"),
            Ordering::Equal => {
                println!("Угадали, поздравляю!!!");
                break;
            }
        }
    }
}
