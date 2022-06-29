use std::io;

use crate::lib::theme::Theme;

#[test]
fn test_space_input() {
    let t1: Theme = Theme::new("s").unwrap();
    let t2: Theme = Theme::new("1").unwrap();
    let t3: Theme = Theme::new("space").unwrap();

    let test: Theme = Theme::Space;

    assert_eq!(t1, test);
    assert_eq!(t2, test);
    assert_eq!(t3, test);
}

#[test]
fn test_food_input() {
    let t1: Theme = Theme::new("f").unwrap();
    let t2: Theme = Theme::new("2").unwrap();
    let t3: Theme = Theme::new("food").unwrap();

    let test: Theme = Theme::Food;

    assert_eq!(t1, test);
    assert_eq!(t2, test);
    assert_eq!(t3, test);
}

#[test]
fn test_websites_input() {
    let t1: Theme = Theme::new("w").unwrap();
    let t2: Theme = Theme::new("3").unwrap();
    let t3: Theme = Theme::new("websites").unwrap();

    let test: Theme = Theme::Websites;

    assert_eq!(t1, test);
    assert_eq!(t2, test);
    assert_eq!(t3, test);
}

#[test]
fn test_animals_input() {
    let t1: Theme = Theme::new("a").unwrap();
    let t2: Theme = Theme::new("4").unwrap();
    let t3: Theme = Theme::new("animals").unwrap();

    let test: Theme = Theme::Animals;

    assert_eq!(t1, test);
    assert_eq!(t2, test);
    assert_eq!(t3, test);
}

#[test]
fn test_countries_input() {
    let t1: Theme = Theme::new("c").unwrap();
    let t2: Theme = Theme::new("5").unwrap();
    let t3: Theme = Theme::new("countries").unwrap();

    let test: Theme = Theme::Countries;

    assert_eq!(t1, test);
    assert_eq!(t2, test);
    assert_eq!(t3, test);
}

#[test]
fn test_invalid_input() {
    let t1: Result<Theme, io::Error> = Theme::new("webs");

    assert!(t1.is_err());
}
