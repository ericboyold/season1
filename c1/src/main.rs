mod pow;

use pow::find_hash_with_prefix;

fn main() {
    let nickname = "Eric";
    println!("============== PoW Implementation ==============");
    println!("Nickname: {}", nickname);

    // task1: Find hash with prefix of 4 zeros
    println!("============== Task 1: Find hash with prefix of 4 zeros ===============");
    find_hash_with_prefix(nickname, 4);
    println!("=======================================================================");

    // task2: Find hash with prefix of 5 zeros
    println!("\n\n=============== Task 2: Find hash with prefix of 5 zeros ==============");
    find_hash_with_prefix(nickname, 5);
    println!("=======================================================================");
}