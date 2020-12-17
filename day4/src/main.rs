use day4::{get_input, parse_input};

fn main() {
    let input = get_input();
    let passports = parse_input(&input);

    let nb_valid_passports_part_one = passports.iter().filter(|p| p.is_valid_part_one()).count();
    println!(
        "\nPart One: We found {} valid passports.",
        nb_valid_passports_part_one
    );

    let nb_valid_passports_part_two = passports.iter().filter(|p| p.is_valid_part_two()).count();
    println!(
        "\nPart Two: We found {} valid passports.",
        nb_valid_passports_part_two
    );
}
