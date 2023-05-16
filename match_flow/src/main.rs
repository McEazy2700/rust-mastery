#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    state: UsState,
    color: Option<String>
}

impl Person {
    fn new(name: String, age: u32, state: UsState, color: Option<String>) -> Self {
        Self {
            age,
            name,
            state,
            color
        }
    }

    fn to_string(self) -> String {
        let color = match self.color {
            None => "None".to_string(),
            Some(color) => color
        };
        format!(
            "Person {{
                name: {},
                age: {},
                state: {:?},
                color: {}
            }}", self.name, self.age, self.state, color
        )
    }
}

impl Coin {
    fn value_in_cents (&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quater(state) => {
                println!("State quater from {:?}:", state);
                25
            }
        }
    }
}


fn main() {
    let my_coin = Coin::Penny;
    let quater = Coin::Quater(UsState::Alaska);
    let dime = Coin::Dime;
    let nickel = Coin::Nickel;
    println!("Dime {:#?}", dime);
    println!("Nickel {:#?}", nickel);
    println!("My coin is worth {} cent(s)", my_coin.value_in_cents());
    println!("My coin is worth {} cent(2)", quater.value_in_cents());

    let frying_pan = Person::new(String::from("Frying Pan"), 25, UsState::Alabama, None);
    println!("{}", frying_pan.to_string());

    let roll = 7;
    match roll {
       3 => println!("You got a 3"),
       7 => println!("Lucky you, you got a 7"),
       other => println!("A regular number {}", other)
    }
}
