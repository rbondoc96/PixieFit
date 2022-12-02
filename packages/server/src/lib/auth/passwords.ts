import crypto from 'crypto';

const PBKDF2_ITERATIONS = 10000;
const PBKDF2_KEY_LEN = 64;
const PBKDF2_DIGEST = 'sha512';

function generatePassword(password: string) {
    const salt = crypto.randomBytes(32).toString('hex');

    // See RFC 8018 5.2
    const hash = crypto
        .pbkdf2Sync(
            password,
            salt,
            PBKDF2_ITERATIONS,
            PBKDF2_KEY_LEN,
            PBKDF2_DIGEST,
        )
        .toString('hex');

    return {
        hash,
        salt,
    };
}

function validatePassword(
    password: string,
    hash: string,
    salt: string,
): boolean {
    const outputHash = crypto
        .pbkdf2Sync(
            password,
            salt,
            PBKDF2_ITERATIONS,
            PBKDF2_KEY_LEN,
            PBKDF2_DIGEST,
        )
        .toString('hex');

    return hash === outputHash;
}

export {generatePassword, validatePassword};
