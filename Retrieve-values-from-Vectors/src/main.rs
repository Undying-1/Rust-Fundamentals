
fn vec_sum(vec: &Vec<i32>) -> i32{

    return vec.iter().sum();
}



fn main(){
    let vec: Vec<i32> = vec![1,2,3,4,5];

    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty"),
    }

    println!("total sum is: {}", vec_sum(&vec))

}

