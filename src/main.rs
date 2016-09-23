
mod dishes {
    pub type Cost = f32;
    pub trait Component {
        fn name(&self) -> String;
        fn cost(&self) -> Cost;
    }

    #[allow(dead_code)]
    pub struct HouseBlend;
    impl Component for HouseBlend {
        fn name(&self) -> String {
            String::from("Кофе (Домашняя смесь)")
        }
        fn cost(&self) -> Cost {
            0.89
        }
    }

    #[allow(dead_code)]
    pub struct DarkRoast;
    impl Component for DarkRoast {
        fn name(&self) -> String {
            String::from("Кофе (Тёмная обжарка)")
        }
        fn cost(&self) -> Cost {
            0.99
        }
    }

    #[allow(dead_code)]
    pub struct Decaf;
    impl Component for Decaf {
        fn name(&self) -> String {
            String::from("Кофе (Без кофеина)")
        }
        fn cost(&self) -> Cost {
            1.05
        }
    }

    #[allow(dead_code)]
    pub struct Espresso;
    impl Component for Espresso {
        fn name(&self) -> String {
            String::from("Кофе (Espresso)")
        }
        fn cost(&self) -> Cost {
            1.99
        }
    }

    #[allow(dead_code)]
    pub struct Milk;
    impl Component for Milk {
        fn name(&self) -> String {
            String::from("Молочная пена")
        }
        fn cost(&self) -> Cost {
            0.10
        }
    }

    #[allow(dead_code)]
    pub struct Mocha;
    impl Component for Mocha {
        fn name(&self) -> String {
            String::from("Шоколад")
        }
        fn cost(&self) -> Cost {
            0.20
        }
    }

    #[allow(dead_code)]
    pub struct Soy;
    impl Component for Soy {
        fn name(&self) -> String {
            String::from("Соя")
        }
        fn cost(&self) -> Cost {
            0.15
        }
    }

    #[allow(dead_code)]
    pub struct Whip;
    impl Component for Whip {
        fn name(&self) -> String {
            String::from("Взбитые сливки")
        }
        fn cost(&self) -> Cost {
            0.10
        }
    }

    #[allow(dead_code)]
    pub struct Dish {
        components: Vec<Box<Component>>,
    }
    impl Dish {
        pub fn new(main: Box<Component>) -> Self {
            Dish { components: vec![main] }
        }
        pub fn add(&mut self, comp: Box<Component>) {
            self.components.push(comp);
        }
        pub fn print(&self) {
            println!("{:8.2} $ = {}", &self.cost(), &self.name());
        }
    }
    impl Component for Dish {
        fn name(&self) -> String {
            if let Some(main) = self.components.iter().take(1).next() {
                return self.components.iter().skip(1).fold(main.name(), |acc, component| {
                    acc + " + " + &component.name()
                });
            }
            String::new()
        }
        fn cost(&self) -> Cost {
            self.components.iter().fold(0.0, |acc, component| acc + component.cost())
        }
    }
}

use dishes::*;

fn main() {
    {
        Dish::new(Box::new(Espresso)).print();
    }

    {
        let mut dish = Dish::new(Box::new(DarkRoast));
        dish.add(Box::new(Mocha));
        dish.add(Box::new(Mocha));
        dish.add(Box::new(Whip));
        dish.print();
    }
    {
        let mut dish = Dish::new(Box::new(HouseBlend));
        dish.add(Box::new(Soy));
        dish.add(Box::new(Mocha));
        dish.add(Box::new(Whip));
        dish.print();
    }
}
