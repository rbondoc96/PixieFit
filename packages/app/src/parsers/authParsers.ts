import {nativeEnum, nullable, object, optional, type output, string} from 'zod';

import Gender from '@/enums/Gender';
import Role from '@/enums/Role';
import {createGetResponseParser} from '@/parsers/responseParsers';
import {iso8601DateTime} from '@/parsers/transforms';

const userSchema = object({
    email: string().email(),
    name: object({
        first: string(),
        last: string(),
        full: string(),
    }),
    role: nativeEnum(Role),
    profile: optional(object({
        id: string(),
        gender: nativeEnum(Gender),
        birthday: iso8601DateTime,
    })),
    last_logged_in_at: nullable(iso8601DateTime),
    created_at: iso8601DateTime,
    updated_at: iso8601DateTime,
});

export type User = output<typeof userSchema>;
export const userParser = createGetResponseParser(userSchema);
