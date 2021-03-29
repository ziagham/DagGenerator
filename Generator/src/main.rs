struct Person;

struct Employee{
    parent: Person
}

impl Person {
    fn do_somthing(&self) {
        println!("Do Somthing!");
    }
}

impl Employee {
    fn do_somtime(&self) {
        println!("Do Somtime!");
    }
}

fn main() {
    let e = Employee {parent: Person };
    e.do_somtime();
    e.parent.do_somthing();
}
