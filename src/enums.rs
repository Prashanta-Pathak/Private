pub fn enumurate() {
    #[derive(Debug)]
    enum CardSuit {
        Hearts,
        Spades,
        Club,
        Diamond,
    }
    #[derive(Debug)]
    struct Card {
        card: String,
        class: CardSuit,
    }

    let first_card: CardSuit = CardSuit::Hearts;
    let mut second_card: CardSuit = CardSuit::Spades;
    second_card = CardSuit::Club;

    let card_details = (CardSuit::Hearts, CardSuit::Club);
    let card_array = [CardSuit::Club, CardSuit::Hearts, CardSuit::Diamond];

    let new_card: Card = Card {
        card: String::from("A"),
        class: CardSuit::Club,
    };

    println!("{:?}", first_card);
    println!("{:?}", new_card);
    println!("{:#?}", second_card);
    println!("{:#?}", card_details);
    println!("{:#?}", card_array);
}
