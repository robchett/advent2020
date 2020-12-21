use std::collections::HashMap;

#[test]
fn test_run_21() {
    let inputs = vec![
        ("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)", (5, 0)),       
    ];
    for i in inputs {
        let res = run(i.0.to_owned());
        match res {
            Ok(o) => assert_eq!(o, i.1),
            Err(e) => panic!(e),
        }
    }  
}

pub fn run(input: String) -> Result<(i64, i64), &'static str> {
    let lines = input.split("\n");
    // Part 1 is the total number of safe igrediets.
    let mut part_1 = 0;
    // Create a hashmap of Allergen -> Possible ingredients
    let mut allergens = HashMap::new();
    // Create a hashmap of Ingredient -> Times seen
    // We'll use this to remove from the part 1 total as we work out each translation
    let mut ingredient_counts = HashMap::new();
    // Create a list of the final translation of each Allergen 
    let mut allergen_translation = vec![];

    for line in lines {
        // Split the line into the Ingredients and Allergens
        let split = line.split(" (contains ",).collect::<Vec<&_>>();
        let line_ingredients = split[0].split(" ").collect::<Vec<&_>>();
        let line_allergens = split[1][0..split[1].len()-1].split(", ").collect::<Vec<&_>>();
        
        // Loop the ingredients and add them to the part 1 total and ingredient totals.
        for i in 0..line_ingredients.len() {
            let ingredient = line_ingredients[i];
            part_1 += 1;    
            ingredient_counts.entry(ingredient.to_owned()).and_modify(|v| *v += 1).or_insert(1);
        }

        // Loop each allergen, for each reduce the set of possible ingredients to the onese that are contained in lines seen before
        for i in 0..line_allergens.len() {
            let a = line_allergens[i];
            let prev_entry = allergens.get(a);
            // If this is the first time seeing the allergen, add all the ingredients to the possible list
            // Otherwise get the union of the existing possibles and new possible
            // Create a blank translation for later use.
            match prev_entry {
                None => {
                    let mut new = vec![];
                    for j in 0..line_ingredients.len() {
                        let ingredient = line_ingredients[j];
                        new.push(ingredient.to_owned());
                    }
                    allergens.insert(a, new);
                    allergen_translation.push((a, "".to_owned()));
                },
                Some(prev) => {
                    let mut new = vec![];
                    for j in 0..line_ingredients.len() {
                        let ingredient = line_ingredients[j];
                        // Only add the items that were seen before for this allergen
                        if prev.contains(&ingredient.to_owned()) {
                            new.push(ingredient.to_owned());
                        }
                    }
                    allergens.insert(a, new);
                }
            }
        }
    }

    // Loop our allergen list looking for items with only one possible option.
    // Make a note for that translation then remove the ingredient from the other allergens list
    // This assumes the list is not ambiguous (thankfully)
    // As we find translations, remove that ingredients count from the part 1 total
    while allergen_translation.iter().filter(|x| x.1 == "").collect::<Vec<&_>>().len() > 0 {
        for i in 0..allergen_translation.len() {
            let a = allergen_translation[i].0;
            if allergens.get(a).unwrap().len() == 1 {
                let at = allergens.get(a).unwrap()[0].to_owned();
                part_1 -= ingredient_counts.get(&at).unwrap();
                allergen_translation[i].1 = at.to_owned();
                for b in 0..allergen_translation.len() {
                    allergens.entry(allergen_translation[b].0).and_modify(|v| {
                        let mut new = vec![];
                        for j in 0..v.len() {
                            if v[j] != at {
                                new.push(v[j].to_owned());
                            }
                        }
                        *v = new;
                    });
                }
            }
        }
    }

    // Part 2 is just sorting our allergens and outputting their translations
    // Annoying that this isn't an integer output like all the others!
    // Sort, map and reduce.
    let mut list = allergen_translation;
    list.sort_by(|x, y| x.0.cmp(&y.0));
    println!("Day 21, Part 2 - {}", list.iter().map(|x| x.1.to_owned()).collect::<Vec<String>>().connect(","));    

    return Ok((part_1 as i64, 0 as i64));
}
