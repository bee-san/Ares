use crate::checkers::checker_object::CheckObject;
use lemmeknow::Data;
use lemmeknow::Identify;

const IDENTIFIER: Identify = Identify {
    min_rarity: None,
    max_rarity: None,
    tags: vec![],
    exclude_tags: vec![],
    file_support: false,
    boundaryless: false,
};

pub fn check_lemmeknow(input: &str) -> Option<CheckObject> {
    // Uses lemmeknow to check if any regexes match
    let lemmeknow_result = IDENTIFIER.identify(input);
    if !lemmeknow_result.is_empty() {
        let return_object = CheckObject {
            is_identified: true,
            text: input,
            checker: "LemmeKnow",
            // Returns a vector of matches
            description: format_data_result(&lemmeknow_result[0].data),
            link: "https://swanandx.github.io/lemmeknow-frontend/",
        };
        return Some(return_object);
    }
    None
}

fn format_data_result(input: &Data) -> String {
    /*
    Input contains these:
        println!("{}", input.Name);
    println!("{}", input.Regex);
    println!("{}", input.plural_name);
    println!("{}", input.Description);
    println!("{}", input.Rarity);
    println!("{}", input.URL);
    println!("{:?}", input.Tags);
    In the future we'd want to include more advanced things like URL. */
    format!("The plaintext is {}", input.Name) // removed .to_string() from here
}
