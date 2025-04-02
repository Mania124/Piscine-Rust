use std::io;

fn main() {
    let riddle = "Riddle: I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut counter = 0;
    loop{
        counter += 1;
        println!("{}", riddle);
        let  mut ans = String::new();
        io::stdin()
        .read_line(&mut ans)
        .expect("failed to read line!");
        ans = ans.trim().to_string();
        if ans==answer{
            println!("{}",counter);
            break;
        }
    }
}
