import {model, Schema, Types} from 'mongoose';
import type {HydratedDocument} from 'mongoose';

interface ExerciseProperties {
    name: string;
    notes: string;
    type: string;
    user_id: Types.ObjectId;
    sets: Types.ObjectId[];
}

type ExerciseDocument = HydratedDocument<ExerciseProperties>;

const ExerciseSchema: Schema = new Schema<ExerciseProperties>(
    {
        name: {
            type: String,
            required: true,
        },
        notes: {
            type: String,
            required: false,
        },
        type: {
            type: String,
            required: true,
            enum: ['Free Weights', 'Bodyweight', 'Cardio', 'TRX', 'Machine'],
        },
        user_id: {
            type: Schema.Types.ObjectId,
            ref: 'User',
            required: true,
        },
        sets: [
            {
                type: Schema.Types.ObjectId,
                ref: 'Set',
            },
        ],
    },
    {
        timestamps: {
            createdAt: 'created_at',
            updatedAt: 'updated_at',
        },
    },
);

export type {ExerciseDocument, ExerciseProperties};
export default model<ExerciseProperties>('Exercise', ExerciseSchema);
