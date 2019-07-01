#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json::Number;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Game {
        gamePk: Number,
    }

    const N: u64 = 20180304170; // max(gamePk) * 10 !
    const INPUT: &str = r#"{"gamePk": 20180304170}"#;

    // https://github.com/serde-rs/json/issues/340
    #[test]
    fn gamePk_number() {
        let json: Game = serde_json::from_str(INPUT).unwrap();
        assert_eq!(
            json,
            Game {
                gamePk: Number::from(N),
            },
        )
    }

    #[test]
    fn gamePk_string() {
        let json: Game = serde_json::from_str(INPUT).unwrap();
        assert_eq!(json.gamePk.to_string(), N.to_string())
    }
}
