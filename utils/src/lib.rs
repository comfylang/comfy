pub fn inc_indent(src: String) -> String {
    "    ".to_owned() + &src.replace("\n", "\n    ")
}

pub fn b<T>(a: T) -> Box<T> {
    Box::new(a)
}
