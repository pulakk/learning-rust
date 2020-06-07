use super::add::*;

#[test]
fn add_two() {
    assert_eq!(sum(1, 2), 3);
}

#[test]
fn add_three() {
    let result = sum(1, 3); 
    assert_ne!(result, 5, "Sum of 1 and 2 is {} and not {}", result, 5);
}

/**
 * `expected` just has to be a
 *  prefix of the actual error msg
*/
#[test]
#[should_panic(expected = "a and b have to be")]
fn add_same_numbers() {
    sum(1, 1);
}

#[test]
fn custom_handle() -> Result<(), String> {
    if sum(1, 2) == 3 {
        Ok(())
    } else {
        Err(String::from("sum(1, 2) is not resulting to 3"))
    }
}