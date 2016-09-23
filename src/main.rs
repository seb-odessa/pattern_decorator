
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
            String::from("House Blend")
        }
        fn cost(&self) -> Cost {
            1.0
        }
    }

    #[allow(dead_code)]
    pub struct DarkRoast;
    impl Component for DarkRoast {
        fn name(&self) -> String {
            String::from("Dark Roast")
        }
        fn cost(&self) -> Cost {
            1.5
        }
    }

    #[allow(dead_code)]
    pub struct Decaf;
    impl Component for Decaf {
        fn name(&self) -> String {
            String::from("Decaf")
        }
        fn cost(&self) -> Cost {
            0.85
        }
    }

    #[allow(dead_code)]
    pub struct Espresso;
    impl Component for Espresso {
        fn name(&self) -> String {
            String::from("Espresso")
        }
        fn cost(&self) -> Cost {
            0.85
        }
    }

    #[allow(dead_code)]
    pub struct Milk;
    impl Component for Milk {
        fn name(&self) -> String {
            String::from("Milk")
        }
        fn cost(&self) -> Cost {
            0.10
        }
    }

    #[allow(dead_code)]
    pub struct Dish {
        components : Vec<Box<Component>>
    }
    impl Dish {
        pub fn new(main : Box<Component>) -> Self {
            Dish {
                components : vec![main]
            }
        }

        pub fn add(&mut self, comp : Box<Component>) {
            self.components.push(comp);
        }
    }
    impl Component for Dish {
        fn name(&self) -> String {
            if let Some(main) = self.components.iter().take(1).next(){
                return self.components.iter().skip(1).fold(
                    main.name(),
                    |acc, component| acc +" + " + &component.name()
                )
            }
            String::new()
        }
        fn cost(&self) -> Cost {
            self.components.iter().fold(
                0.0,
                |acc, component| acc + component.cost())
        }
    }
}

use dishes::*;

fn main() {

    let mut dish = Dish::new(Box::new(Espresso));
    dish.add(Box::new(Milk));


    println!("{:8.2} $ = {}", &dish.cost(), &dish.name());
}
