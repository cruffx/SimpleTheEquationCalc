use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();
    println!("Programm written by ! Cruffx#6666 on Rust - www.cruffx.space\n");

    loop {
    
    println!("Решите уравнение");

    println!("Введите а:");
    match io::stdin().read_line(&mut a_str){
        Ok(_) => {},
        Err(e) => println!("Ошибка - {}", e)
    }

    println!("Введите b:");
    match io::stdin().read_line(&mut b_str){
        Ok(_) => {},
        Err(e) => println!("Ошибка - {}", e)
    }

    println!("Введите c:");
    match io::stdin().read_line(&mut c_str){
        Ok(_) => {},
        Err(e) => println!("Ошибка - {}", e)
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b*b) - 4.0 * (a * c);

    if d > 0.0 {
        let x1 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2 = ((-b) - d.sqrt()) / (2.0 * a);

        println!("Решено\nЕсть 2 корня\nD = {}\nКорень1 = {}\nКорень 2 = {}", d, x1, x2);
    }

    if d == 0.0 {
        let x = (-b) / (2.0 * a);

        println!("Решено\nЕсть 1 корень\nD = 0\nКорень = {}", x);
    }

    if d < 0.0 {
        println!("Корней не существует!\nD < 0\nD = {}", d);
    }
    }

}
//COPYRIGHT BY CRUFFX
//COPYRIGHT BY CRUFFX
//COPYRIGHT BY CRUFFX
//COPYRIGHT BY CRUFFX
//COPYRIGHT BY CRUFFX