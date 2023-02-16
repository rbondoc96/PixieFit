import {model, Schema, Types} from 'mongoose';
import type {HydratedDocument} from 'mongoose';

interface SetProperties {
    type?: string;
    repetitions?: number;
    weight?: number; // Determined by user units setting
    duration_ms?: number;
    rest_duration_ms?: number;
    user_id: Types.ObjectId;
    exercise_id: Types.ObjectId;
    workout_id: Types.ObjectId;
}

type SetDocument = HydratedDocument<SetProperties>;

const SetSchema: Schema = new Schema<SetProperties>(
    {
        type: {
            type: String,
            enum: ['Drop Set', 'Warm-Up', 'Failure'],
            required: false,
        },
        repetitions: {
            type: Number,
            required: false,
        },
        weight: {
            type: Number,
            required: false,
        },
        duration_ms: {
            type: Number,
            required: false,
        },
        rest_duration_ms: {
            type: Number,
            required: false,
        },
        user_id: {
            type: Schema.Types.ObjectId,
            ref: 'User',
            required: true,
        },
        exercise_id: {
            type: Schema.Types.ObjectId,
            ref: 'Exercise',
            required: true,
        },
        workout_id: {
            type: Schema.Types.ObjectId,
            ref: 'Workout',
            required: true,
        },
    },
    {
        timestamps: {
            createdAt: 'created_at',
            updatedAt: 'updated_at',
        },
    },
);

export type {SetDocument, SetProperties};
export default model<SetProperties>('Set', SetSchema);
