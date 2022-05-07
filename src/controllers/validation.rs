
pub fn password_is_valid(password: &str) -> bool {

    let digits = [0,1,2,3,4,5,6,7,8,9];

    password.len() > 8 && digits.iter().any(|x| password.contains(x.to_string().as_str()))
}



pub fn phone_number_is_valid(phone_number: &String) -> bool {
    let digits = ['0','1','2','3','4','5','6','7','8','9'];

    phone_number.chars().all(|x| digits.contains(&x))
}

