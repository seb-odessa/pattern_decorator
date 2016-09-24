
mod beverage {
    pub type Cost = f32;
    pub trait Beverage {
        fn name(&self) -> String;
        fn cost(&self) -> Cost;
    }

    #[allow(dead_code)]
    pub struct HouseBlend;
    impl Beverage for HouseBlend {
        fn name(&self) -> String {
            String::from("Кофе (Домашняя смесь)")
        }
        fn cost(&self) -> Cost {
            0.89
        }
    }

    #[allow(dead_code)]
    pub struct DarkRoast;
    impl Beverage for DarkRoast {
        fn name(&self) -> String {
            String::from("Кофе (Тёмная обжарка)")
        }
        fn cost(&self) -> Cost {
            0.99
        }
    }

    #[allow(dead_code)]
    pub struct Decaf;
    impl Beverage for Decaf {
        fn name(&self) -> String {
            String::from("Кофе (Без кофеина)")
        }
        fn cost(&self) -> Cost {
            1.05
        }
    }

    #[allow(dead_code)]
    pub struct Espresso;
    impl Beverage for Espresso {
        fn name(&self) -> String {
            String::from("Кофе (Espresso)")
        }
        fn cost(&self) -> Cost {
            1.99
        }
    }

    #[allow(dead_code)]
    pub struct Milk <'a>{
        comp : Box<Beverage +'a>
    }
    impl <'a> Milk <'a>{
        pub fn new<T: Beverage +'a>(comp:T) -> Self {
            Milk { comp : Box::new(comp) }
        }
    }
    impl <'a> Beverage for Milk <'a>{
        fn name(&self) -> String {
            self.comp.name() + &String::from(" + Молочная пена")
        }
        fn cost(&self) -> Cost {
            self.comp.cost() + 0.10
        }
    }

    #[allow(dead_code)]
    pub struct Mocha<'a> {
        comp : Box<Beverage + 'a>
    }
    impl <'a> Mocha <'a>{
        pub fn new<T: Beverage +'a>(comp:T) -> Self {
            Mocha { comp : Box::new(comp) }
        }
    }
    impl <'a> Beverage for Mocha <'a>{
        fn name(&self) -> String {
            self.comp.name() + &String::from(" + Шоколад")
        }
        fn cost(&self) -> Cost {
            self.comp.cost() + 0.20
        }
    }

    #[allow(dead_code)]
    pub struct Soy<'a>{
        comp : Box<Beverage +'a>
    }
    impl <'a> Soy <'a>{
        pub fn new<T: Beverage +'a>(comp:T) -> Self {
            Soy { comp : Box::new(comp) }
        }
    }
    impl <'a> Beverage for Soy <'a>{
        fn name(&self) -> String {
            self.comp.name() + &String::from(" + Соя")
        }
        fn cost(&self) -> Cost {
            self.comp.cost() + 0.15
        }
    }

    #[allow(dead_code)]
    pub struct Whip<'a> {
        comp : Box<Beverage +'a>
    }
    impl <'a> Whip <'a>{
        pub fn new<T: Beverage +'a>(comp:T) -> Self {
            Whip { comp : Box::new(comp) }
        }
    }
    impl <'a> Beverage for Whip<'a> {
        fn name(&self) -> String {
            self.comp.name() + &String::from(" + Взбитые сливки")
        }
        fn cost(&self) -> Cost {
            self.comp.cost() + 0.10
        }
    }
}

use beverage::*;

pub fn print<T: Beverage>(beverage : &T) {
    println!("{:8.2} $ = {}", &beverage.cost(), &beverage.name());
}

fn main() {
    print(&Espresso);
    print(&Milk::new(Espresso));
    print(&Whip::new(Mocha::new(Mocha::new(DarkRoast))));
    print(&Whip::new(Mocha::new(Soy::new(HouseBlend))));
}
