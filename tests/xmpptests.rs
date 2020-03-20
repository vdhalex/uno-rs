#[cfg(test)]
mod test_server {
    use uno::game_rules::unostate::UnoState;
    use uno::game_rules::GameState;
    use uno::xmpp::servexmpp;

    #[test]
    fn parse_xml_test() {
        let xml = r#"<message id="jejeje"><last-card>a839</last-card></message>"#;
        assert_eq!(servexmpp::parse_xml(xml.as_bytes()), vec!["a839"]);
    }

    #[test]
    fn parse_xml_test_full() {
        let xml = r#"<message><player>3</player><last-card>a839</last-card><deck-encrypted>ioejfiowjefoijvoijvoiewjvweoijv</deck-encrypted><current-action-state>none</current-action-state></message>"#;
        assert_eq!(
            servexmpp::parse_xml(xml.as_bytes()),
            vec!["3", "a839", "ioejfiowjefoijvoijvoiewjvweoijv", "none"]
        );
    }

    #[test]
    fn parse_xml_test_empty() {
        let xml = r#"<message></message>"#;
        assert!(servexmpp::parse_xml(xml.as_bytes()).is_empty());
    }

    #[test]
    fn to_xml_test() {
        let gs: UnoState = GameState::new();
        let xml = r#"<player>1</player><last-card>28</last-card><deck-encrypted>1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 </deck-encrypted><current-action-state>none</current-action-state>"#;

        assert_eq!(gs.to_xml(), xml);
    }
}
