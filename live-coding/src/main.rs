trait Honk {
    fn honk(&self) -> String;
}

trait Name {
    fn name() -> &'static str;
}

struct Car(u64);

impl Honk for Car {
    fn honk(&self) -> String {
        format!("Tut-tut {}", self.0).into()
    }
}

impl Name for Car {
    fn name() -> &'static str {
        "Car"
    }
}

struct Bicycle;

impl Honk for Bicycle {
    fn honk(&self) -> String {
        "Dring-dring".into()
    }
}

impl Name for Bicycle {
    fn name() -> &'static str {
        "Bicycle"
    }
}

fn show_honk(vec: &Vec<&dyn Honk>) {
    for honk in vec.iter() {
        println!("{}", honk.honk());
    }
}

use prettytable::*;
use std::collections::HashMap;

fn insert<'a, 'b, T : Honk + Name>(m: &'a mut HashMap<&str, &'b dyn Honk>, h: &'b T) {
     m.insert(T::name(), h);
}

fn show_types(m: &HashMap<&str, &dyn Honk>) {
    let mut table = Table::new();

    table.add_row(row!["Vehicle Type", "Honk"]);

    for (k, v) in m.iter() {
        table.add_row(row![k, v.honk()]);
    }

    table.printstd();
}

fn main() {
    let car = Car(42);
    let car2 = Car(65);

    // println!("{}", car.honk());

    let bc = Bicycle;

    let vec: Vec<&dyn Honk> = vec![&car, &car2, &bc];

    show_honk(&vec);

    let mut m: HashMap<_, &dyn Honk> = HashMap::new();

    insert(&mut m, &car);
    insert(&mut m, &bc);

    show_types(&m);
}
