import {model, Schema} from 'mongoose';
import type {HydratedDocument} from 'mongoose';
import uniqueValidator from 'mongoose-unique-validator';

import emailValidator from '@/lib/validators/email';

interface UserProperties {
    admin: boolean;
    first_name: string;
    last_name: string;
    email: string;
    password?: string;
    salt?: string;
    uses_imperial_units?: boolean;
}

type UserDocument = HydratedDocument<UserProperties>;

const UserSchema: Schema = new Schema<UserProperties>(
    {
        admin: {
            type: Boolean,
            default: false,
        },
        first_name: {
            type: String,
            required: true,
        },
        last_name: {
            type: String,
            required: true,
        },
        email: {
            type: String,
            validate: {
                validator: emailValidator,
                message: 'Please provide a valid e-mail address.',
            },
            required: true,
            unique: true,
        },
        password: {
            type: String,
        },
        salt: {
            type: String,
        },
        uses_imperial_units: {
            type: Boolean,
            default: true,
        },
    },
    {
        timestamps: {
            createdAt: 'created_at',
            updatedAt: 'updated_at',
        },
    },
);

UserSchema.plugin(uniqueValidator);

export type {UserDocument, UserProperties};
export default model<UserProperties>('User', UserSchema);
