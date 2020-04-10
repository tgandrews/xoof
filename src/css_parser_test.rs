use css_parser::*;

#[test]
fn it_parses_element_selector_with_attribute() {
    let style_sheet = parse_css("div { height: 100px; }".to_string());
    let rule: &Rule = style_sheet.rules.get(0).unwrap();
    let selector: &Selector = rule.selectors.get(0).unwrap();
    match selector {
        Selector::Simple(s) => {
            assert_eq!(s.tag_name.clone().unwrap(), String::from("div"));
        }
    }
    let declaration: &Declaration = rule.declarations.get(0).unwrap();
    assert_eq!(declaration.name, String::from("height"));
    match declaration.value {
        Value::Length(amount, unit) => {
            assert_eq!(amount, 100_f32);
            let unit_result = match unit {
                Unit::Px => "px",
            };
            assert_eq!(unit_result, "px")
        }
        _ => assert_eq!("Value Length", "Value not length"),
    }
}

fn parse_css(style: String) -> StyleSheet {
    // let mut warnings = vec![];
    let style_sheet = parse(style);
    // for warn in &warnings {
    //     println!("Warn: {}", warn)
    // }
    // assert_eq!(warnings.len(), 0, "No warnings expected");
    return style_sheet.clone();
}
