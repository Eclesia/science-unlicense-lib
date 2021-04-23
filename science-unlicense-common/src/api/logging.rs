//
// Public Domain - unlicense.science
//

///
///
pub fn format(message: &str, value: &dyn ToString) -> String {
    return String::from(message).replace("{}", &value.to_string());
}
