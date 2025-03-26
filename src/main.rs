use std::ops;

struct Foo;
struct Bar;
#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;
// The `std::opp::Add` trait is used to sepcify the functionaltiey of `+`
// Here, we make `Add<Bar>` - the trait for addition with RHS of type `Foo`
// This block implements teh opration: Bar + Foo = BarFOo

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> Self::Output {
        println!("> Bar.add(Foo) was Called");
        FooBar
    }
}
// The `std::opp::Add` trait is used to sepcify the functionaltiey of `+`
// Here, we make `Add<Bar>` - the trait for addition with RHS of type `Foo`
// This block implements teh opration: Bar + Foo = BarFOo

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> Self::Output {
        println!("> Bar.add(Foo) was Called");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
    println!("Hello, world!");
}
