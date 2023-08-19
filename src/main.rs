use rand::Rng;

fn main() {
    println!("\x1b[2J");
    let mut arr = [[" "; 106]; 50];
    let mut flag = 0;
    let letters = ".,/)(><^?_+-%':&][#$!abcdefghijklmnopqrstuvwxyzABCDIFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let _chars : Vec<&str> = letters.split_inclusive(|_| { true }).collect();
    loop {
        println!("\x1b[H");
        let _first = 0;
        if flag == 0 {
            let mut cf = vec![];
            for _i in 0..106 {
                cf.push((rand::thread_rng().gen_range(1..50), rand::thread_rng().gen_range(1..75)))
            }
            for (x, i) in arr.iter().enumerate() {
                for (y, _j) in i.iter().enumerate() {
                    if cf[y].0 >= x && cf[y].1 - x <= x {
                        //arr[x - 1][y] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                        //arr[x][y] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                        for i in arr.iter() {
                            println!("{}", i.join(" "));
                        }
                        println!("\x1b[H")
                    }
                }
            }
        flag = 1
        } else {
            println!("pass");
        }
    }
}
