pub fn pretty_print<T>(value: &T)
where
    T: serde::Serialize,
{
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}
