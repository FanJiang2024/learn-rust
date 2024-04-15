#[test]
fn trait_starter() {

  // 定义一组方法的签名（可以不提供具体实现）
  // 定义若干“行为”
  // 可以在trait内部定义常量
  trait Greeter {
    const NUM: f64 = 3.14;
    fn greet(&self);
    fn hello();
  }

  struct Person {
    name: String,
  }

  impl Person {
    fn new(name: String) -> Self {
      Self {
        name
      }
    }
  }

  impl Greeter for Person {
    fn greet(&self) {
        println!("greet from {}", self.name);
    }

    fn hello() {
        println!("Hello! {}", <Self as Greeter>::NUM);
    }
  }

  let tom = Person::new("Tom".to_string());
  tom.greet();
  Person::hello();

}