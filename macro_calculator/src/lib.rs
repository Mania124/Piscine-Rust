use json::*;

pub struct Food {
    // expected public fields
    #[allow(dead_code)]
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let portions = food.nbr_of_portions;
        let cal_str = &food.calories.1;
        let cal_val: f64 = cal_str.replace("kcal", "").parse().unwrap_or(0.0);

        total_cals += cal_val * portions;
        total_carbs += food.carbs * portions;
        total_proteins += food.proteins * portions;
        total_fats += food.fats * portions;
    }

    object! {
        "cals" => round_f64_smart(total_cals),
        "carbs" => round_f64_smart(total_carbs),
        "proteins" => round_f64_smart(total_proteins),
        "fats" => round_f64_smart(total_fats),
    }
}
pub fn round_f64_smart(val: f64) -> String {
    let rounded = (val * 100.0).round() / 100.0;
    if (rounded * 10.0) % 1.0 == 0.0 {
        format!("{:.1}", rounded)
    } else {
        format!("{:.2}", rounded)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let foods = [
//         Food {
//             name: "big mac".to_owned(),
//             calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
//             proteins: 27.,
//             fats: 26.,
//             carbs: 41.,
//             nbr_of_portions: 2.,
//         },
//         Food {
//             name: "pizza margherita".to_owned(),
//             calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
//             proteins: 13.89,
//             fats: 11.21,
//             carbs: 49.07,
//             nbr_of_portions: 4.9,
//         },
//     ];
//         let result = calculate_macros(&foods);
//         let comparison = object!{
//             "cals": 2777.39,
//             "carbs": 322.44,
//             "proteins": 122.06,
//             "fats": 106.93
//         };
//         assert_eq!(result, comparison);
//     }
// }
