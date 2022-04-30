use crate::cmd::IngredientOpts;
use crate::SubCommand::Ingredient;

enum IngredientAction{
    Add,
    Remove,
    Update,
    List,
}

//TODO: complete this data structure and add impls
struct Inventory{

}

impl Inventory{

}
/// Ingredient is the interface function that serves as the dispatch point to other functionality.
pub fn ingredient(input: &String, debug: bool, opt_type: &String, args: &IngredientOpts) {
    println!("Inspect called for {}", input);

    // If set override the log level
    if debug {
        println!("{:#?}", args);
    }
    // Set the container type from argument string.
    let ingredient_type = match opt_type.as_ref() {
        "add" => IngredientAction::Add,
        "remove" => IngredientAction::Remove,
        "update" => IngredientAction::Update,
        "list" => IngredientAction::List,
        _ => IngredientAction::Empty,
    };

    // Match will determine the correct function call to make based on the ENUM passed.
    match ingredient_type {
        IngredientAction::Add => add_ingredient(),
        IngredientAction::Remove => remove_ingredient(),
        IngredientAction::Update => update_ingredient(),
        IngredientAction::List => list_ingredient(),
        IngredientAction::Empty => create_empty(),
    };

}

// Group of functions which will manage ingredients.
fn add_ingredient() {}

fn remove_ingredient() {}

fn update_ingredient() {}

fn list_ingredient() {}

fn create_empty() {}