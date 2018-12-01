use eval::LispType;

pub fn add(arguments: &Vec<LispType>) -> LispType {
    let res = arguments
        .iter()
        .fold(0f64, |acc, x| acc + x.clone().unwrap_number());

    return LispType::Num(res);
}
