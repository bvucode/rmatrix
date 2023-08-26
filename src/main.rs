use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    println!("\x1b[2J");
    const W: usize = 106;
    const H: usize = 50;
    let mut arr = [[" "; W]; H];
    let mut flag = 0;
    const LETTERS = ".,/)(><^?_+-%':&][#$!abcdefghijklmnopqrstuvwxyzABCDIFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let _chars : Vec<&str> = LETTERS.split_inclusive(|_| { true }).collect();
    loop {
        println!("\x1b[H");
        let mut start = 0;
        if flag == 0 {
            let mut cf = vec![];
            for _i in 0..W {
                cf.push((rand::thread_rng().gen_range(1..H), rand::thread_rng().gen_range(1..75)))
            }
            for x in 0..H {
                for y in 0..W {
                    if cf[y].0 >= x && cf[y].1 - x <= x {
                        arr[x - 1][y] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                        arr[x][y] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                        for i in arr.iter() {
                            println!("{}", i.join(" "));
                        }
                        thread::sleep(Duration::from_millis(4));
                        println!("\x1b[H");
                    }
                }
            }
            flag = 1;
        } else {
            for i in 0..W {
                let mut c = 0;
                for j in 0..H {
                    if arr[j][i] == " " {
                        c += 1;
                    }
                }
                if c == H {
                    let rn = rand::thread_rng().gen_range(16..32);
                    let rr = rand::thread_rng().gen_range(1..16);
                    let r = rn - rr;
                    for k in 0..r {
                        arr[rr + (k + 1)][i] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                    }
                }
            }
            for i in 0..W {
                for j in 0..H {
                    if arr[j][i] != " " && start == 0 {
                        arr[j][i] = " ";
                        start = 1;
                    } else if arr[j][i] == " " && start == 1 {
                        arr[j][i] = _chars[rand::thread_rng().gen_range(0.._chars.len())];
                        start = 0;
                    }
                }
                for i in arr.iter() {
                    println!("{}", i.join(" "));
                }
                thread::sleep(Duration::from_millis(4));
                println!("\x1b[H");
            }
        }
    }
}
