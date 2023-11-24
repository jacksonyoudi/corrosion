// struct A {
//     f1: u32,
//     f2: u32,
//     f3: u32,
// }
//
//
// fn foo(a: &mut A) -> &u32 { &a.f2 }
// fn bar(a: &mut A) -> u32 { a.f1 + a.f3 }
//
// fn baz(a: &mut A) {
//     // The later usage of x causes a to be borrowed for the rest of the function.
//     let x = foo(a);
//     // Borrow checker error:
//     // let y = bar(a); // ~ ERROR: cannot borrow `*a` as mutable more than once
//     //          at a time
//     println!("{}", x);
// }



struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

// These functions take a B or C, rather than A.
fn foo(b: &mut B) -> &u32 { &b.f2 }
fn bar(c: &mut C) -> u32 { c.f1 + c.f3 }

fn baz(a: &mut A) {
    let x = foo(&mut a.b);
    // Now it's OK!
    let y = bar(&mut a.c);
    println!("{}", x);
}


fn main() {

}