use rand::Rng;  // 引入亂數生成函式庫
use std::cmp::Ordering;  // 引入比較函式庫
use std::io;  // 引入標準輸入輸出函式庫

fn main() {
    println!("猜數字遊戲開始！");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // 產生 1~100 之間的亂數

    loop {
        println!("請輸入您的猜測：");

        let mut guess = String::new();  // 宣告一個可變的空字串，用來儲存玩家的猜測

        io::stdin()
            .read_line(&mut guess)  // 讀取玩家輸入的內容，並存到 guess 變數中
            .expect("讀取輸入失敗！");

        let guess: u32 = match guess.trim().parse() {  // 將玩家輸入的內容轉換成整數型態，並存到 guess 變數中
            Ok(num) => num,
            Err(_) => continue,  // 若轉換失敗則繼續下一輪迴圈
        };

        println!("你猜測的數字是：{}", guess);

        match guess.cmp(&secret_number) {  // 比較玩家猜測的數字和隨機產生的數字的大小
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("你贏了！");
                break;  // 結束遊戲
            }
        }
    }
}
