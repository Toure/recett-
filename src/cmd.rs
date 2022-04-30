pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "recette",
author = "Toure Dunnon <tdunnon@gmail.com>",
about = "Recette is a simple recipe manager, allowing the user to add and track ingredients as well as\
recipes."
)]

// Here we describe the basic interface to the functions that will
// carry out the tasks.
pub struct Cmd {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    pub debug: bool,
    pub opt_type: String,
    pub input: String,
    #[structopt(short, long, global = true, help = "file path to recipe template.")]
    pub file: String,
    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

// This ENUM creates an entrypoint for the subcommand structure.
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "ingredient", about = "Manage ingredient inventory.")]
    Ingredient(IngredientOpts),
    #[structopt(name = "recipe", about = "Manage your collection of recipes.")]
    Recipe(RecipeOpts),
}

// Below here is where all the subcommand structures are defined.
#[derive(Debug, StructOpt)]
pub struct IngredientOpts {
    #[structopt(short, long, help = "Add new items to your inventory, which can be added as a list of strings or single item.\
    ex: recette ingredient add milk || add milk, flour, eggs")]
    pub add: Option<Vec<String>>,
    #[structopt(short, long, help = "Remove an item from the inventory. ex: recette ingredient remove eggs")]
    pub remove: Option<Vec<String>>,
    #[structopt(short, long, help = "Update the quantiy of an item within the inventory. example: recette ingredient update -q 10 eggs")]
    pub update: Option<Vec<String>>,
    #[structopt(short, long, help = "List current ingredient inventory.")]
    pub list: bool,
}

#[derive(Debug, StructOpt)]
pub struct RecipeOpts {
    #[structopt(short, long, help = "Add new recipes to the catalog ex: recette recipe -f <recipe template> add 'chicken soup'")]
    pub add: Option<Vec<String>>,
    #[structopt(short, long, help = "Remove an recipes from the catalog. ex: recette recipe remove eggs")]
    pub remove: Option<Vec<String>>,
    #[structopt(short, long, help = "Update the quantiy of an items within the recipe catalog. example: recette recipe update -q 10 eggs")]
    pub update: Option<Vec<String>>,
    #[structopt(short, long, help = "Make will print a list of simple instructions for a recipe and remove the items from inventory.")]
    pub make: Option<String>,
    #[structopt(short, long, help = "List current ingredient inventory.")]
    pub list: bool,
}