use nom::{
    bytes::complete::tag,
    character::{complete::char, streaming::alpha1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Ingredient<'a>(&'a str);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Allergen<'a>(&'a str);

struct Food<'a> {
    ingredients: Vec<Ingredient<'a>>,
    allergens: Vec<Allergen<'a>>,
}

fn parse_food<'a>(i: &'a str) -> IResult<&'a str, Food<'a>> {
    map(
        tuple((
            separated_list1(char(' '), map(alpha1, Ingredient)),
            delimited(
                tag(" (contains "),
                separated_list1(tag(", "), map(alpha1, Allergen)),
                char(')'),
            ),
        )),
        |(ingredients, allergens)| Food {
            ingredients,
            allergens,
        },
    )(i)
}

pub fn run() {
    let text = std::fs::read_to_string("input/day21.txt").unwrap();
    let foods: Vec<_> = text
        .lines()
        .map(|s| all_consuming(parse_food)(s).unwrap().1)
        .collect();

    let all_ingredients: HashSet<_> = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().cloned())
        .collect();
    let all_allergens: HashSet<_> = foods
        .iter()
        .flat_map(|food| food.allergens.iter().cloned())
        .collect();

    let mut allergen_ingredient = HashMap::new();
    let mut ingredient_allergen = HashMap::new();
    while allergen_ingredient.len() < all_allergens.len() {
        for allergen in foods.iter().flat_map(|food| food.allergens.iter()) {
            if allergen_ingredient.contains_key(allergen) {
                continue;
            }

            let possible_ingredients: HashSet<Ingredient> = all_ingredients
                .iter()
                .cloned()
                .filter(|i| !ingredient_allergen.contains_key(i))
                .collect();

            let possible_ingredients = foods
                .iter()
                .filter(|food| food.allergens.iter().find(|&a| a == allergen).is_some())
                .fold(possible_ingredients, |acc, food| {
                    acc.intersection(&food.ingredients.iter().cloned().collect())
                        .cloned()
                        .collect()
                });

            if possible_ingredients.len() == 1 {
                let ingredient = possible_ingredients.iter().cloned().next().unwrap();
                allergen_ingredient.insert(allergen, ingredient);
                ingredient_allergen.insert(ingredient, allergen);
            }
        }
    }

    println!(
        "day21: count of ingredients with no allergens is {}",
        foods
            .iter()
            .flat_map(|food| food.ingredients.iter())
            .filter(|i| !ingredient_allergen.contains_key(i))
            .count()
    );

    let mut dangerous_ingredients: Vec<_> = all_ingredients
        .iter()
        .filter(|i| ingredient_allergen.contains_key(i))
        .collect();
    dangerous_ingredients.sort_by(|a, b| {
        ingredient_allergen
            .get(a)
            .unwrap()
            .0
            .cmp(ingredient_allergen.get(b).unwrap().0)
    });
    let dangerous_ingredients: Vec<_> = dangerous_ingredients.iter().map(|i| i.0).collect();
    println!(
        "day21: dangerous ingredients: {}",
        dangerous_ingredients.join(",")
    );
}
