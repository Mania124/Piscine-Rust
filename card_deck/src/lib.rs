use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::rng();
        let val = rng.random_range(1..5);
        Self::translate(val)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::rng();
        let val = rng.random_range(1..14);
        Self::translate(val)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            i @ 2..=10 => Rank::Number(i),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card
        == Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let your_card = Card {
//             rank: Rank::random(),
//             suit: Suit::random(),
//         };
//         let card_deck = Card{
//             suit: Suit::Spade,
//             rank: Rank::Ace,
//         };
//         let result = card_deck::winner_card(your_card);
//         assert_eq!(result, true||false);
//     }
// }
