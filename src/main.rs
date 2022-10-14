use std::io::stdin;

/*
TODO: добавить комментарии
*/

fn main() {
    println!("Введите год");
    let year: i32 = read_var();
    if year % 4 == 0 {
        println!("Год {year} високосный");
    } else {
        println!("Год {year} не високосный");
    }
    pause();
}

fn read_var() -> i32 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: i32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная Х должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}

fn pause() { //фукция паузы
    println!("нажмите Enter чтобы выйти.");
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
}