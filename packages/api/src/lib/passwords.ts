import bcrypt from 'bcrypt';
import passwordValidator from 'password-validator';

import ValidationException from '@/exceptions/ValidationException';

const schema = new passwordValidator();

// prettier-ignore
schema
    .is().min(parseInt(env('PASSWORD_MIN_LENGTH')))
    .is().max(parseInt(env('PASSWORD_MAX_LENGTH')))
    .has().uppercase(1, 'Password must have at least 1 uppercase letter.')
    .has().lowercase(1, 'Password must have at least 1 lowercase letter.')
    .has().digits(parseInt(env('PASSWORD_MIN_DIGITS')))
    // The symbols being checked can be found in the repo
    // https://github.com/tarunbatra/password-validator/blob/master/src/constants.js
    // Regex: ([`~\\!@#\\$%\\^\\&\\*\\(\\)\\-_\\=\\+\\[\\\{\\}\\]\\\\\|;:\\\'",<.>\\/\\?€£¥₹§±].*)
    .has().symbols(1, 'Password must contain at least one special character (e.g. ~, !, @).')
    .has().not().spaces();

export async function encrypt(password: string): Promise<string> {
    const saltRounds = parseInt(env('PASSWORD_SALT_ROUNDS'));

    const salt = await bcrypt.genSalt(saltRounds);

    return await bcrypt.hash(password, salt);
}

export async function compare(password: string, hash: string): Promise<boolean> {
    return await bcrypt.compare(password, hash);
}

interface PasswordValidationError {
    validation: string;
    arguments?: number;
    inverted?: boolean;
    message: string;
}

export async function validatePasswordPattern(password: string): Promise<void> {
    // Setting these options will make it so an array of errors is returned instead of a boolean.
    // The error object was defined from testing invalid passwords in Postman.
    const errors = schema.validate(password, {
        details: true,
    }) as PasswordValidationError[];

    if (errors.length > 0) {
        throw new ValidationException(
            'password',
            errors.map(error => error.message),
            'Invalid password.',
        );
    }
}
