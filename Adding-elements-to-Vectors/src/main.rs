use std::io;

fn add_item(vec: &mut Vec<i32>, item: &i32){
    vec.insert(0, *item);
    vec.push(*item)
}

fn con_two_vectors(vec: &mut Vec<i32>, vec_2: &Vec<i32>) -> Vec<i32>{
    vec.extend(vec_2);
    return  vec.to_vec();
}


fn main() {
    let mut vec = vec![1,2,3];
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error while reading!");

    //add_item(&mut vec, &input.trim().parse::<i32>().unwrap());

    //println!("{:?}", vec);

    let new_vec: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", new_vec);

    con_two_vectors(&mut vec, &new_vec);

    println!("{:?}", vec)
}
