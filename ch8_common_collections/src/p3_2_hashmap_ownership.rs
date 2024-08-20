use std::collections::HashMap;

const FIELD_NAME: &str = "Favorite color";

#[allow(unused_variables, dead_code)]
pub fn main() {
    {
        let field_name = String::from(FIELD_NAME);
        let field_value = String::from("Blue");

        let map = HashMap::from([(field_name, field_value)]);

        // владение перешло к map

        // borrow of moved value: `field_name`
        // println!("{}", field_name)
        // borrow of moved value: `field_value`
        // println!("{}", field_value);
    }

    {
        let field_name = String::from(FIELD_NAME);
        let field_value = String::from("Blue");

        // map не владеет field_value
        // но время жизни HashMap
        // !!!***ограничено временем жизни field_value***!!!
        // поэтому реже используется

        let map = HashMap::from([(field_name, &field_value)]);

        println!("{}", field_value);

        println!(
            "{}",
            map.get(&String::from(FIELD_NAME))
                .copied()
                .unwrap_or(&String::from("None"))
        );
    }
}
