///使用可切片类型或者胖指针类型没有限制。
/// 事实上，你应该总是用借用类型（borrowed type）, 而不是自有数据类型的借用（borrowing the owned type）。
/// 例如&str 而非 &String, &[T] 而非 &Vec<T>, 或者 &T 而非 &Box<T>.
///
/// 当自有数据结构（owned type）的实例已经提供了一个访问数据的间接层时，使用借用类型可以让你避免增加间接层。
/// 举例来说，String类型有一层间接层，所以&String将有两个间接层。
/// 我们可以用&Str来避免这种情况，无论何时调用函数，强制&String转换为&Str。
///
///
fn main() {
    say_hello("youdi");
}


fn say_hello(name: &str) -> String {
    // 变成 String
    // to_owned 新建一个String类型,然后将 str clone 到String中
    // let string = "hello".to_owned();

    format!("hello {}", name)

}