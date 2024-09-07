use rust_pig_latin::pig_latin;

fn main() {
    let origin = "it is my new car";
    let pig_latin = pig_latin::translate(origin);

    println!("Origin: {}", origin);
    println!("Pig Latin: {}", pig_latin);
}
