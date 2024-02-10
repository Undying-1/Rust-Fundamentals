use std::io::{self, Read};

struct Vowels {
    a: i16,
    e: i16,
    i: i16,
    o: i16,
    u: i16,
    
}


impl Vowels {
    fn update_vowel_counter(&mut self, c: char){
        match c {
            'a' => self.a += 1,
            'e' => self.e += 1,
            'i' => self.i += 1,
            'o' => self.o += 1,
            'u' => self.u += 1,
            _ => return
        }
    }

    fn total_counter(&self){
        println!("a: {}", self.a);
        println!("e: {}", self.e);
        println!("i: {}", self.i);
        println!("o: {}", self.o);
        println!("u: {}", self.u);
    }
}


fn get_longest_word(sentence: &String) -> String{
    let arr: Vec<&str> = sentence.split_whitespace().collect();
    let mut max = String::from("");
    for &i in arr.iter(){
        if i.len() >= max.len(){
            max = i.to_string();
        }
    }
    return  max;
}


fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog";
    println!("{}", &sentence[0..=4]);

    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    let mut vowels_counter = Vowels{
        a:0,
        e:0,
        i:0,
        o:0,
        u:0,
    };

    for c in sentence.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels_counter.update_vowel_counter(c),
            _ => continue,
        }
    }

    vowels_counter.total_counter();

    

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error while reading");

    println!("the longest word is: {}", get_longest_word(&input));

}