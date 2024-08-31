use std::collections::{HashMap, VecDeque};


struct Solution;

impl Solution {
    pub fn length_of_longest_substring(&self, s: String) -> i32{
        let mut word_dict: HashMap<char, i32> = HashMap::new();
        let mut char_stack = VecDeque::new();
        let mut latest_winner = VecDeque::new();
        for  ch in s.chars(){
            let v =  word_dict.get(& ch).unwrap_or(&0) ;
                    if *v > 0 {


                        if char_stack.len() > latest_winner.len(){
                            latest_winner = char_stack.clone();
                        }
                        while char_stack[0] != ch {
                            let head_char = char_stack.pop_front(); 
                           
                            match head_char {
                                Some(hch) => {
                                    if hch == ch {
                                        break;}
                                    else {
                                        if let Some(v) = word_dict.get_mut(&hch) {
                                            *v -= 1;
                                        }
        
                                    }
                                        },
                                None => {break;}

                            }
                        println!("Char: {:?}", ch);
                        println!("Head char: {:?}", head_char);
                        println!("Char stack Reduced: {:?}", char_stack);
                        println!("Word Dict: {:?}", word_dict);
                        }
                        char_stack.pop_front();
                        char_stack.push_back(ch);

                        println!("Char stack: {:?}", char_stack);
                        println!("Latest winner: {:?}", latest_winner);
                        // char_stack.push_back(ch);
                        // *word_dict.entry(ch).or_insert(0) += 1;

                        }

                        else {
                            char_stack.push_back(ch);
                            *word_dict.entry(ch).or_insert(0) += 1;
                            println!("Added Char stack: {:?}", char_stack);
        

                    }

                
            
        }
        if char_stack.len() > latest_winner.len(){
            latest_winner = char_stack.clone();
        }
        latest_winner.len() as i32

    }
}


fn main() {
    let sol = Solution{};
    // let s = "abcabcbb".to_string();
    let s = "jbpnbwwd".to_string();
    let result = sol.length_of_longest_substring(s);
    println!("Result: {}", result);
}


// use std::collections::{HashMap, VecDeque};

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let mut word_dict: HashMap<char, i32> = HashMap::new();
//         let mut char_stack = VecDeque::new();
//         let mut max_length = 0;

//         s.chars().for_each(|ch| {
//             let count = word_dict.entry(ch).or_insert(0);
//             *count += 1;

//             if *count > 1 {
//                 while let Some(&front) = char_stack.front() {
//                     if front == ch {
//                         char_stack.pop_front();
//                         break;
//                     }
//                     if let Some(val) = word_dict.get_mut(&front) {
//                         *val -= 1;
//                     }
//                     char_stack.pop_front();
//                 }
//             }

//             char_stack.push_back(ch);
//             max_length = max_length.max(char_stack.len());
//         });

//         max_length as i32
//     }
// }
