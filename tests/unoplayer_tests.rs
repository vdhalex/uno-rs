use uno::player::GamePlayer;
use uno::player::unoplayer::{UnoPlayer, UnoCard, ColorType, CardType};

#[test]
fn player_with_no_cards_test() {
    let mut new_player = UnoPlayer::new();
    assert_eq!(new_player.show_cards(),[]);
    assert_eq!(new_player.show_cards().len(),0);
}

#[test]
fn adding_one_card_test() {
    let mut new_player = UnoPlayer::new();
    let mut cards_to_add = Vec::new();

    // check adding empty vector doesn't change player's hand
    new_player.add_cards(&mut cards_to_add);
    assert_eq!(new_player.show_cards(),[]);
    assert_eq!(new_player.show_cards().len(),0);

    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
    new_player.add_cards(&mut cards_to_add);

    // check one card was successfully added
    assert_eq!(new_player.show_cards(), [UnoCard::new(ColorType::Red, CardType::Number(2))]);
    assert_eq!(new_player.show_cards().len(),1);
}

#[test]
fn adding_multiple_cards_of_all_kinds_test() {
    let mut new_player = UnoPlayer::new();
    let mut cards_to_add = Vec::new();

    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
    cards_to_add.push(UnoCard::new(ColorType::Yellow, CardType::Skipcard));
    cards_to_add.push(UnoCard::new(ColorType::Blue, CardType::Reversecard));
    cards_to_add.push(UnoCard::new(ColorType::Green, CardType::Draw2card));
    cards_to_add.push(UnoCard::new(ColorType::None, CardType::Wildcard));
    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Wildcard4));
    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::None));
    new_player.add_cards(&mut cards_to_add);

    assert_eq!(new_player.show_cards(), [UnoCard::new(ColorType::Red, CardType::Number(2)),
                                        UnoCard::new(ColorType::Yellow, CardType::Skipcard),
                                        UnoCard::new(ColorType::Blue, CardType::Reversecard),
                                        UnoCard::new(ColorType::Green, CardType::Draw2card),
                                        UnoCard::new(ColorType::None, CardType::Wildcard),
                                        UnoCard::new(ColorType::Red, CardType::Wildcard4),
                                        UnoCard::new(ColorType::Red, CardType::None)]);
    assert_eq!(new_player.show_cards().len(),7);
}

#[test]
fn deleting_only_card_in_deck_test() {
    let mut new_player = UnoPlayer::new();
    let mut cards_to_add = Vec::new();

    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
    new_player.add_cards(&mut cards_to_add);
    new_player.remove_card(&UnoCard::new(ColorType::Red, CardType::Number(2)));

    // check player's hand is empty after removing only card
    assert_eq!(new_player.show_cards(),[]);
    assert_eq!(new_player.show_cards().len(),0);
}

#[test]
fn deleting_one_card_in_deck() {
    let mut new_player = UnoPlayer::new();
    let mut cards_to_add = Vec::new();

    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
    cards_to_add.push(UnoCard::new(ColorType::Yellow, CardType::Skipcard));
    cards_to_add.push(UnoCard::new(ColorType::Blue, CardType::Reversecard));
    new_player.add_cards(&mut cards_to_add);
    new_player.remove_card(&UnoCard::new(ColorType::Yellow, CardType::Skipcard));

    // check player's hand no longer has deleted card but others remain
    assert_eq!(new_player.show_cards(),[UnoCard::new(ColorType::Red, CardType::Number(2)),
                                        UnoCard::new(ColorType::Blue, CardType::Reversecard)]);
    assert_eq!(new_player.show_cards().len(),2);
}

#[test]
fn deleting_card_not_in_deck_doesnt_change_deck_test() {
    let mut new_player = UnoPlayer::new();
    let mut cards_to_add = Vec::new();

    cards_to_add.push(UnoCard::new(ColorType::Red, CardType::Number(2)));
    cards_to_add.push(UnoCard::new(ColorType::Yellow, CardType::Skipcard));
    cards_to_add.push(UnoCard::new(ColorType::Blue, CardType::Reversecard));
    new_player.add_cards(&mut cards_to_add);
    new_player.remove_card(&UnoCard::new(ColorType::Green, CardType::Draw2card));

    // check player's hand is empty after removing only card
    assert_eq!(new_player.show_cards(),[UnoCard::new(ColorType::Red, CardType::Number(2)),
                                        UnoCard::new(ColorType::Yellow, CardType::Skipcard),
                                        UnoCard::new(ColorType::Blue, CardType::Reversecard)]);
    assert_eq!(new_player.show_cards().len(),3);
}