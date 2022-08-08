// use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
/// 
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Card{
    number: u8,
    figure: Figure,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Figure{
    Heart,
    Spade,
    Diamond,
    Clover,
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Score{
    HighCard,
    OnePair,
    TwoPair,
    ThreeCard,
    Straight,
    Flush,
    FullHouse,
    FourCard,
    StraightFlush,
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut highest_score = Score::HighCard;
    let mut highest_made: Vec<u8> = Vec::new();
    let mut highest_vec: Vec<u8> = Vec::new();
    for hand in hands {
        let (tmp_score, tmp_made_cards, tmp_vec) = find_class_of_hand(hand);
        if highest_score < tmp_score {
            highest_score = tmp_score;
            highest_made = tmp_made_cards;
            highest_vec = tmp_vec;
        } else if highest_score == tmp_score {
            if highest_made < tmp_made_cards {
                highest_made = tmp_made_cards;
                highest_vec = tmp_vec;
            }
            else if highest_made == tmp_made_cards {
                if highest_vec < tmp_vec {
                    highest_vec = tmp_vec;
                }
            }
        }
        
    }

    let mut res: Vec<&str> = Vec::new();
    
    for hand in hands {
        let (tmp_score, _, tmp_vec) = find_class_of_hand(hand);
        if highest_score == tmp_score &&  highest_vec == tmp_vec {
            res.push(hand);
        } 
    }
    res
}

// fn cards_to_hand(Cards :Vec<Card>) -> String {
//     let mut res = String::new();
//     for i in Cards {
//         res.push_str(&i.number.to_string());
//         match i.figure {
//             Figure::Heart => res.push('H'),
//             Figure::Spade => res.push('S'),
//             Figure::Diamond => res.push('D'),
//             Figure::Clover => res.push('C'),
//         }
//     }
//     res
// }

fn extract_card(card: &str) -> Card {
    let num: u8;
    let fig: Figure;
    
    let a = card.chars().nth(0).unwrap();
    let b = card.chars().nth(1).unwrap();
    match b {
        '0' => {
            num = 10;
            fig = match card.chars().nth(2).unwrap() {
                'H' => Figure::Heart,
                'S' => Figure::Spade,
                'D' => Figure::Diamond,
                'C' => Figure::Clover,
                _ => Figure::Clover,
            };
        }
        _ => {
            num = match a {
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => a.to_digit(10).unwrap() as u8,
            };
            fig = match b {
                'H' => Figure::Heart,
                'S' => Figure::Spade,
                'D' => Figure::Diamond,
                'C' => Figure::Clover,
                _ => Figure::Clover,
            };
        }
    }

    Card { number: num, figure: fig }
}

fn hand_to_cards(hand: &str) -> Vec<Card> {
    let mut res: Vec<Card> = Vec::new();
    for i in hand.split(' '){
        res.push(extract_card(&i));
    }
    assert!(res.len() == 5);
    res
}

fn find_class_of_hand(hand: &str) -> (Score, Vec<u8>, Vec<u8>) {
    let mut cards = hand_to_cards(hand);
    assert!(cards.len() == 5);
    cards.sort();
    let num0 = cards[0].number;
    let fig0 = cards[0].figure;
    let num1 = cards[1].number;
    let fig1 = cards[1].figure;
    let num2 = cards[2].number;
    let fig2 = cards[2].figure;
    let num3 = cards[3].number;
    let fig3 = cards[3].figure;
    let num4 = cards[4].number;
    let fig4 = cards[4].figure;

    let vec = vec![num4, num3, num2, num1, num0];
    
    let mut score: Score;
    let mut made_cards: Vec<u8> = Vec::new();

    if (fig0 == fig1 && fig1 == fig2 && fig2 == fig3 && fig3 == fig4) &&
    (num1 - num0 == 1 && num2 - num1 == 1 && num3 - num2 == 1 && num4 - num3 == 1) {
        score = Score::StraightFlush;
        made_cards = vec.clone();
    }
    else if num0 == num1 && num1 == num2 && num2 == num3 {
        score = Score::FourCard;
        made_cards.push(num0);
    }
    else if num3 == num4 && num1 == num2 && num2 == num3 {
        score = Score::FourCard;
        made_cards.push(num1);
    }
    else if (num0 == num1 && num1 == num2) && (num3 == num4) {
        score = Score::FullHouse;
        made_cards = vec![num0, num4];
    }
    else if (num0 == num1) && (num2 == num3 && num3 == num4) {
        score = Score::FullHouse;
        made_cards = vec![num4, num0];
    }
    else if fig0 == fig1 && fig1 == fig2 && fig2 == fig3 && fig3 == fig4 {
        score = Score::Flush;
        made_cards = vec.clone();
    }
    else if num1 - num0 == 1 && num2 - num1 == 1 && num3 - num2 == 1 && num4 - num3 == 1 {
        score = Score::Straight;
        made_cards = vec.clone();
    }
    else if num0 == num1 && num2 == num3 {
        score = Score::TwoPair;
        made_cards = vec![num3, num0];
    }
    else if num0 == num1 && num3 == num4 {
        score = Score::TwoPair;
        made_cards = vec![num4, num0];
    }
    else if num1 == num2 && num3 == num4 {
        score = Score::TwoPair;
        made_cards = vec![num4, num1];
    }
    else if num0 == num1 && num1 == num2 {
        score = Score::ThreeCard;
        made_cards.push(num2);
    }
    else if num1 == num2 && num2 == num3 {
        score = Score::ThreeCard;
        made_cards.push(num2);
    }
    else if num2 == num3 && num3 == num4 {
        score = Score::ThreeCard;
        made_cards.push(num2);
    }

    else if num0 == num1 {
        score = Score::OnePair;
        made_cards.push(num0);
    }
    else if num1 == num2 {
        score = Score::OnePair;
        made_cards.push(num1);
    }
    else if num2 == num3 {
        score = Score::OnePair;
        made_cards.push(num2);
    }
    else if num3 == num4 {
        score = Score::OnePair;
        made_cards.push(num3);
    }
    else {
        score = Score::HighCard;
        made_cards.push(num4);
    }
    
    if num4 == 14{
        if (fig0 == fig1 && fig1 == fig2 && fig2 == fig3 && fig3 == fig4) &&
        vec == vec![14, 5, 4, 3, 2] {
            score = Score::StraightFlush;
            made_cards = vec![1,2,3,4,5];
        }
        else if vec == vec![14, 5, 4, 3, 2] {
            score = Score::Straight;
            made_cards = vec![1,2,3,4,5];
        }
    }
    return (score, made_cards, vec);
}