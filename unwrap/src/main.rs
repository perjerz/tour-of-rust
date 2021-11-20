fn do_side_effect(num: i8) -> Result<i8, String> {
    if num > 60 {
        return Ok(num+1);
    }
    return Err(String::from("bad path"));
 }

fn main() -> Result<(), String> {
    let res = do_side_effect(61).unwrap();
    println!("{}", res);
    Ok(())
}
