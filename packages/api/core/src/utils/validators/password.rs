use super::ValidatorResult;
use std::collections::HashMap;

pub struct CharacterValidator {
    validator: Box<dyn Fn(&char) -> bool>,
    is_satisfied: bool,
    did_error: bool,
}

impl CharacterValidator {
    pub fn new(validator: fn(&char) -> bool) -> Self {
        Self {
            validator: Box::new(validator),
            is_satisfied: false,
            did_error: false,
        }
    }

    pub fn validate(&mut self, c: &char) -> bool {
        let result = (self.validator)(c);
        self.is_satisfied = result;

        result
    }
}

pub fn password(password: &str) -> ValidatorResult {
    let mut one_off_validators: HashMap<String, CharacterValidator> = HashMap::new();
    let mut repeat_validators: HashMap<String, CharacterValidator> = HashMap::new();

    let mut validation_errors: Vec<String> = Vec::new();

    one_off_validators.insert(
        "Must contain an uppercase letter".to_string(),
        CharacterValidator::new(|c| c.is_uppercase()),
    );

    one_off_validators.insert(
        "Must contain a lowercase letter".to_string(),
        CharacterValidator::new(|c| c.is_lowercase()),
    );

    one_off_validators.insert(
        "Must contain a number".to_string(),
        CharacterValidator::new(|c| c.is_numeric()),
    );

    one_off_validators.insert(
        "Must contain a special character".to_string(),
        CharacterValidator::new(|c| {
            vec!['!', '@', '#', '$', '%', '^', '&', '*', '-', '_', '=', '+'].contains(c)
        }),
    );

    repeat_validators.insert(
        "Must not contain a space".to_string(),
        CharacterValidator::new(|c| c != &' '),
    );

    if password.len() < 8 {
        validation_errors.push("Password is too short".to_string());
    }

    if password.len() > 32 {
        validation_errors.push("Password is too long".to_string());
    }

    let chars = password.chars();
    for c in chars {
        for (_, validator) in one_off_validators.iter_mut() {
            if !validator.is_satisfied {
                validator.validate(&c);
            }
        }

        for (message, validator) in repeat_validators.iter_mut() {
            let is_valid = validator.validate(&c);

            if !is_valid && !validator.did_error {
                validation_errors.push(message.to_owned());
                validator.did_error = true;
            }
        }
    }

    one_off_validators
        .iter()
        .filter(|(_, validator)| !(validator.is_satisfied))
        .for_each(|(message, _)| validation_errors.push(message.to_owned()));

    if !validation_errors.is_empty() {
        return ValidatorResult::Invalid(validation_errors);
    }

    ValidatorResult::Valid
}
