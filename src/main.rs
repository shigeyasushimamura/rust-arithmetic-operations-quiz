use rand::Rng;
fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::thread_rng().gen_range(1..=2);

        match quiz_mode {
            1 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("op1のアドレス:{:p}", &op1);

                println!("{} + {} = ??", op1, op2);
                println!("??の値を入力してください");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().ok();
                dbg!(ans_input);

                match ans_input {
                    Some(ans_input) => {
                        if dbg!(ans_input == op1 + op2) {
                            println!("正解!");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解!");
                        }
                    }
                    None => {
                        println!("入力をi32型に変換できませんでした")
                    }
                }
            },
            2 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("op1のアドレス:{:p}", &op1);

                println!("{} - {} = ??", op1, op2);
                println!("??の値を入力してください");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();

                let ans_input = ans_input.trim().parse::<i32>().ok();
                dbg!(ans_input);

                match ans_input {
                    Some(ans_input) => {
                        if dbg!(ans_input == op1 - op2) {
                            println!("正解!");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解!");
                        }
                    }
                    None => {
                        println!("入力をi32型に変換できませんでした")
                    }
                }
            },

            _ => unreachable!(),
        }
    }

    println!("クリア！");
    println!("i32 が扱えるデータ範囲: {} ~ {}", i32::MIN, i32::MAX);
    println!("u32 が扱えるデータ範囲: {} ~ {}", u32::MIN, u32::MAX);
}
