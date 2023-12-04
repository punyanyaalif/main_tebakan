use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Tebak angka!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Masukkan tebakan Anda!");

        let mut tebakan = String::new();

        io::stdin().read_line(&mut tebakan).expect("Failed to read line!");

        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tebakan Anda : {tebakan}");

        match tebakan.cmp(&secret_number){
            Ordering::Less => println!("Kurangg!"),
            Ordering::Greater => println!("Kegedeannn!"),
            Ordering::Equal => {println!("Kamu Benar...!");
            break;
            }
        }
    }
}
