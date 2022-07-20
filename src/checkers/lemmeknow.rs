use lemmeknow::Identify;
use crate::checkers::checkerObject::{CheckObject};
use lemmeknow::Data;

pub fn check_lemmeknow(input: &str) -> Option<CheckObject> {
    // Uses lemmeknow to check if any regexes match
    let identifier = Identify::default();
    let lemmeKnowResult = identifier.identify(input);
    if !lemmeKnowResult.is_empty(){
        let returnObject = CheckObject{
            is_identified: true,
            text: input.to_string(),
            checker: "LemmeKnow".to_string(),
            // Returns a vector of matches
            description: formatDataResult(lemmeKnowResult[0].data.to_owned()),
            link: "https://swanandx.github.io/lemmeknow-frontend/".to_string(),
        };
        return Some(returnObject);
    }
    None
}

fn formatDataResult(input: Data) -> String{
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
    format!("The plaintext is {}", input.Name).to_string()
}