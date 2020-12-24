use std::collections::{HashMap, HashSet};

type Ingredient = &'static str;
type Allergen = &'static str;

pub struct Food {
    ingredients: HashSet<Ingredient>,
    guaranteed_allergens: HashSet<Allergen>,
}

pub struct ParseResult {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
    foods: Vec<Food>,
}

struct PossibleIngredientAllergens(HashMap<Ingredient, HashSet<Allergen>>);

impl PossibleIngredientAllergens {
    fn compute(input: &ParseResult) -> Self {
        // all allergens are possible by default
        let mut possible_allergens: HashMap<_, _> = input
            .ingredients
            .iter()
            .map(|&i| (i, input.allergens.clone()))
            .collect();

        for food in &input.foods {
            let ingredients_not_in_food = food.ingredients.difference(&input.ingredients);
            for &ing in ingredients_not_in_food {
                possible_allergens.insert(
                    ing,
                    possible_allergens[ing]
                        .iter()
                        .filter(|&&a| !food.guaranteed_allergens.contains(a))
                        .copied()
                        .collect(),
                );
            }
        }

        PossibleIngredientAllergens(possible_allergens)
    }

    fn count_ingredients_without_any_allergens(&self) -> usize {
        self.0
            .iter()
            .filter(|(_, allergens)| allergens.is_empty())
            .count()
    }
}

pub fn parse(input: &[&'static str]) -> ParseResult {
    let mut ingredients = HashSet::new();
    let mut allergens = HashSet::new();
    let mut foods = Vec::new();

    for &line in input {
        let mut split = line.split(" (");
        let food_ingredients: HashSet<Ingredient> = split.next().unwrap().split(' ').collect();
        let food_allergens: HashSet<Allergen> = split
            .next()
            .map_or(HashSet::new(), |s| s[..s.len() - 1].split(", ").collect());

        ingredients.extend(food_ingredients.iter());
        allergens.extend(food_allergens.iter());
        foods.push(Food {
            ingredients: food_ingredients,
            guaranteed_allergens: food_allergens,
        });
    }

    ParseResult {
        ingredients,
        allergens,
        foods,
    }
}

fn main() {
    let input = get_input();
    let parse_result = parse(&input);
    let possible_allergens = PossibleIngredientAllergens::compute(&parse_result);
    println!(
        "solution is {}",
        possible_allergens.count_ingredients_without_any_allergens()
    );
}

pub fn get_input() -> Vec<&'static str> {
    vec![]
}
