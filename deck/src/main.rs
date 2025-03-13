#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // list of suits
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = ["ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen", "king"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        
        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {deck:#?}");
}
