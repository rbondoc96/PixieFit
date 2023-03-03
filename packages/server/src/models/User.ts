import bcrypt from 'bcrypt';
import {model, Schema} from 'mongoose';
import type {HydratedDocument} from 'mongoose';
import uniqueValidator from 'mongoose-unique-validator';

import emailValidator from '@/lib/validators/email';

interface UserProperties {
    admin: boolean;
    birthday: string;
    email: string;
    first_name: string;
    goal?: string;
    height_cm: number;
    last_name: string;
    password: string;
    sex: string;
    use_metric_units: boolean;
    email_verified_at: Date;
    created_at: Date;
    updated_at: Date;
}

type UserDocument = HydratedDocument<UserProperties>;

const userSchema: Schema = new Schema<UserProperties>(
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
            required: [true, 'An e-mail address is required.'],
            unique: true,
        },
        password: {
            type: String,
            required: true,
            minLength: [6, 'Password must be at least 6 characters long.'],
        },
        sex: {
            type: String,
            required: true,
            enum: ['Male', 'Female'],
        },
        birthday: {
            type: String,
            required: true,
        },
        height_cm: {
            type: Number,
            required: true,
        },
        goal: {
            type: String,
            required: false,
        },
        use_metric_units: {
            type: Boolean,
            default: false,
            required: true,
        },
        email_verified_at: {
            type: Date,
            nullable: true,
            default: null,
            required: false,
        },
    },
    {
        timestamps: {
            createdAt: 'created_at',
            updatedAt: 'updated_at',
        },
    },
);

userSchema.plugin(uniqueValidator);

userSchema.pre('save', async function (next) {
    // eslint-disable-next-line no-invalid-this
    this.password = await bcrypt.hash(this.password, 10);
    next();
});

export type {UserDocument, UserProperties};
export default model<UserProperties>('User', userSchema);
