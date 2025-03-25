struct Sheep {
    naked: bool,
    name: &'static str,
}
trait Animal {
    // Associated function signature; `Self` refes to the implemetor type
    fn new(name: &'static str) -> Self;
    // Method signatures; these will return a string
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    // these can provide defualt methond definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            // implementor methods can use implementor's trait methods.
            println!("{} is alredy neked", self.name());
        } else {
            println!("{} gets a hairtuct!", self.name);
            self.naked = true;
        }
    }
}
// implint Animal for Sheep
impl Animal for Sheep {
    // `Self` is implementor type: `Sheep`
    fn new(in_name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: in_name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "beeeeeeeh!!"
        }
    }
    // Defualt trait method can overridden
    fn talk(&self) {
        // for example we can add some quite contemplation.
        println!("{} pouses briefly....{}", self.name, self.noise());
    }
}
fn main() {
    // type annotations is necessary in theis cause.
    let mut dolly: Sheep = Animal::new("Dooly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
