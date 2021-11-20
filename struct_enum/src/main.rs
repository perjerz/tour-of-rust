enum Gender {
    Male,
    Female,
    NotSpecified,
}

struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
}

fn main() {
    let p1 = Person {
        first_name: String::from("Chris"),
        last_name: String::from("Redfield"),
        age: 30,
        gender: Gender::Male,
    };
    println!("Hello, {} {}, you are {} yeasr old.!", p1.first_name, p1.last_name, p1.age);
    match p1.gender {
        Gender::Male => println!("You are male."),
        Gender::Female => println!("You are female."),
        Gender::NotSpecified => println!("You don't want to talke about gender."),
    }
}
