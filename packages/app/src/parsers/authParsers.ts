import {object, type output, string} from 'zod';

import {createGetResponseParser} from '@/parsers/responseParsers';

const authenticatedUserSchema = object({
    email: string().email(),
    name: object({
        first: string(),
        last: string(),
        full: string(),
    }),
    created_at: string(),
    updated_at: string(),
});

const registeredUserSchema = object({
    id: string(),
    first_name: string(),
    last_name: string(),
}).transform(raw => ({
    id: raw.id,
    name: `${raw.first_name} ${raw.last_name}`,
    firstName: raw.first_name,
    lastName: raw.last_name,
}));

export type AuthenticatedUser = output<typeof authenticatedUserSchema>;
export type RegisteredUser = output<typeof registeredUserSchema>;
export const authenticatedUserParser = createGetResponseParser(authenticatedUserSchema);
export const registeredUserParser = createGetResponseParser(registeredUserSchema);
