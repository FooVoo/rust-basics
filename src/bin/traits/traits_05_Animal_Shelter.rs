pub mod problem {
    // TODO: Create an Animal trait with methods:
    // - make_sound(&self) -> String
    // - get_name(&self) -> &str
    // - feed(&mut self)
    // - is_hungry(&self) -> bool

    trait Animal {
        fn make_sound(&self) -> String;
        fn get_name(&self) -> &str;
        fn feed(&mut self);
        fn is_hungry(&self) -> bool;
    }

    struct Dog {
        name: String,
        is_hungry: bool,
    }

    struct Cat {
        name: String,
        is_hungry: bool,
    }

    struct Bird {
        name: String,
        is_hungry: bool,
    }

    // TODO: Implement Animal for Dog, Cat, and Bird

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            "Woof!".to_string()
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn feed(&mut self) {
            self.is_hungry = false;
        }

        fn is_hungry(&self) -> bool {
            self.is_hungry
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            "Meow!".to_string()
        }
        fn get_name(&self) -> &str {
            &self.name
        }
        fn feed(&mut self) {
            self.is_hungry = false;
        }
        fn is_hungry(&self) -> bool {
            self.is_hungry
        }
    }

    impl Animal for Bird {
        fn make_sound(&self) -> String {
            "Chirp!".to_string()
        }
        fn get_name(&self) -> &str {
            &self.name
        }
        fn feed(&mut self) {
            self.is_hungry = false;
        }
        fn is_hungry(&self) -> bool {
            self.is_hungry
        }
    }

    // TODO: Create a Shelter struct that can hold different types of animals
    // struct Shelter {
    //     animals: Vec<???>, // What type should this be?
    // }
    struct Shelter<'a> {
        animals: Vec<&'a mut dyn Animal>,
    }

    // TODO: Implement these methods for Shelter:
    // impl Shelter {
    //     fn new() -> Self { ... }
    //     fn add_animal(&mut self, animal: ???) { ... }
    //     fn feed_all(&mut self) { ... }
    //     fn make_all_sounds(&self) { ... }
    //     fn count_hungry(&self) -> usize { ... }
    // }

    impl<'a> Shelter<'a> {
        fn new(animals: Option<Vec<&'a mut dyn Animal>>) -> Self {
            if let Some(animalsAlready) = animals {
               return Shelter {
                     animals: animalsAlready,
               }
            }
            Shelter {
                animals: Vec::new(),
            }
        }

        fn add_animal<'b>(&'a mut self, animal: &'b mut dyn Animal)
        where
            'b: 'a,
        {
            self.animals.push(animal);
        }

        fn feed_all(&mut self) {
            for animal in &mut self.animals {
                animal.feed();
                animal.make_sound();
            }
        }

        fn make_all_sounds(&self) {
            for animal in &self.animals {
                println!("{} says: {}", animal.get_name(), animal.make_sound());
            }
        }

        fn count_hungry(&self) -> usize {
            self.animals.iter().filter(|a| a.is_hungry()).count()
        }
    }

    pub(crate) fn main() {
        let mut shelter = Shelter::new(vec![
            Dog {
                name: "Buddy".to_string(),
                is_hungry: true,
            },
            Cat {
                name: "Whiskers".to_string(),
                is_hungry: true,
            },
            Bird {
                name: "Tweety".to_string(),
                is_hungry: true,
            },
        ]);

        shelter.add_animal(mut Cat {
            name: "Whiskers".to_string(),
            is_hungry: true,
        });

        shelter.add_animal(&mut Bird {
            name: "Tweety".to_string(),
            is_hungry: true,
        });

        shelter.make_all_sounds();
        let hungry_count = shelter.count_hungry();
        println!("Number of hungry animals: {}", hungry_count);

        shelter.feed_all();
        let hungry_count_after_feeding = shelter.count_hungry();
        println!(
            "Number of hungry animals after feeding: {}",
            hungry_count_after_feeding
        );
    }
}
