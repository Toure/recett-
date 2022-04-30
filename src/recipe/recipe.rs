use crate::cmd::RecipeOpts;
use crate::SubCommand::Recipe;

enum RecipeAction{
    Add,
    Remove,
    Update,
    List,
}

//TODO: complete this data structure and add impls
struct Recipe{

}

impl Recipe {

}

/// Recipe is the interface function that serves as the dispatch point to other functionality.
pub fn recipe(input: &String, debug: bool, opt_type: &String, args: &RecipeOpts) {
    println!("Inspect called for {}", input);

    // If set override the log level
    if debug {
        println!("{:#?}", args);
    }
    // Set the container type from argument string.
    let ingredient_type = match opt_type.as_ref() {
        "add" => RecipeAction::Add,
        "remove" => RecipeAction::Remove,
        "update" => RecipeAction::Update,
        "list" => RecipeAction::List,
        _ => RecipeAction::Empty,
    };

    // Match will determine the correct function call to make based on the ENUM passed.
    match ingredient_type {
        RecipeAction::Add => add_recipe(),
        RecipeAction::Remove => remove_recipe(),
        RecipeAction::Update => update_recipe(),
        RecipeAction::List => list_recipe(),
        RecipeAction::Empty => create_empty(),
    };

}

// Group of functions which will manage ingredients.
fn add_recipe() {}

fn remove_recipe() {}

fn update_recipe() {}

fn list_recipe() {}

fn create_empty() {}