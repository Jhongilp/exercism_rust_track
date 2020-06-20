fn main() {
    let result = reverse_text("learning Rust");
    println!("Results: {}", result);
}

fn reverse_text(input: &str) -> String {    
    input.chars().rev().collect()
}

// fn reverse_text(input: &str) -> String {
//     let mut text: Vec<_> = input.split(" ").collect();
//     text.reverse();
//     println!("reverse: {:?}", text);
    
//     let mut new_reverse = vec![];
//     for word in text {
        
//         let mut reverse_word: Vec<_> = word.split("").collect();
//         println!("{}", word);
//         reverse_word.reverse();
//         let word_join = reverse_word.join("");
//         println!("reverse word: {}", word_join);
//         new_reverse.push(word_join);
//     }
//     println!("new_reverse: {:?}", new_reverse);
//     let word_join = new_reverse.join(" ");
//     word_join
// }