fn main() {
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(8);
    i32_vec.push(8);
    i32_vec.push(8);

    let mut float_vec = Vec::new();
    float_vec.push(0.1);
    float_vec.push(0.1);
    float_vec.push(0.1);

    let string_vec = vec![String::from("World"), String::from("Hello")];
    for str in string_vec.iter() {
        println!("{}", str)
    }

    let mut i8_vec = vec![1,2,3];
    i8_vec.push(4);
    for i in i8_vec {
        println!("{:?}", i)
    }
}
