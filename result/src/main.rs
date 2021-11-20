fn is_some_thing(something: String) -> Result<String, String> {
    match something.as_str() {
        "Something" => Ok(something),
        _ => Err(String::from("Nothing")),
    }
}


fn main() {
    let thing = is_some_thing(String::from("Something"));
    match thing { 
        //
        Ok(ref v) => String::from("Somthing is ") + v.as_str(),
        Err(_) => String::from("Nothing on you baby"),
    };

    println!("{:?}", thing);
}
