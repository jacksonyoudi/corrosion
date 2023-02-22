fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_give_back(s2);
}


fn gives_ownership() -> String {
    return String::from("hello");
}

fn takes_and_give_back(a: String) -> String {
    a
}


fn swap<'a>(mut a: &'a mut i32, mut b: &'a mut i32) {
    // (a, b) = (b, a)
    let t = a;
    a = b;
    b = t;
}