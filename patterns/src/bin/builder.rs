/// 通过调用建造者来构造对象。
/// 当你需要很多不同的构造器或者构造器有副作用的时候这个模式会有帮助。
/// 优点
// 将构造方法与其他方法分开。
//
// 防止构造器数量过多。
//
// 即使构造器本身很复杂，也可以做到封装后一行初始化。
//
// 缺点
// 与直接构造一个结构体或者一个简单的构造函数相比，这种方法太复杂。
#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}


impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}


#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        FooBuilder {
            bar: String::from("x"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo {
            bar: self.bar
        }
    }
}

fn main() {
    let f = Foo {
        bar: String::from("Y")
    };
    let y = Foo::builder().name(String::from("Y")).build();
    let z = FooBuilder::new()
        .name(String::from("X"))
        .build();
    assert_eq!(f, y);
}