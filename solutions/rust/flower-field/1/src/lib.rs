use std::char::from_digit;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut res = Vec::new();
    for (row_counter, row) in garden.iter().enumerate() {
        let mut new_row = String::new();
        for (plant_counter, plant) in row.chars().enumerate() {
            let mut total_this_plant = 0;
            if plant == ' ' {
                let row_counter_testable = row_counter > 0;
                let plant_counter_testable = plant_counter > 0;

                if row_counter_testable
                    && let Some(top_row) = garden.get(row_counter - 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if row_counter_testable
                    && let Some(top_row) = garden.get(row_counter - 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter + 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if row_counter_testable
                    && plant_counter_testable
                    && let Some(top_row) = garden.get(row_counter - 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter - 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if let Some(top_row) = garden.get(row_counter)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter + 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if plant_counter_testable
                    && let Some(top_row) = garden.get(row_counter)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter - 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if let Some(top_row) = garden.get(row_counter + 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if let Some(top_row) = garden.get(row_counter + 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter + 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }

                if plant_counter_testable
                    && let Some(top_row) = garden.get(row_counter + 1)
                    && let Some(top_plant) = top_row.chars().nth(plant_counter - 1)
                    && top_plant == '*'
                {
                    total_this_plant += 1;
                }
                let current_char = from_digit(total_this_plant, 10).expect("Error");
                if current_char == '0' {
                    new_row.push(' ');
                } else {
                    new_row.push(current_char);
                }
            } else {
                new_row.push(plant)
            }
        }
        res.push(new_row);
    }
    res
}