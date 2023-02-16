import {model, Schema, Types} from 'mongoose';
import type {HydratedDocument} from 'mongoose';

interface WorkoutProperties {
    user_id: Types.ObjectId;
    title: string;
    notes?: string;
    start: Date;
    end: Date;
    duration_ms: number;
    exercises: Types.ObjectId[];
}

type WorkoutDocument = HydratedDocument<WorkoutProperties>;

const WorkoutSchema: Schema = new Schema<WorkoutProperties>(
    {
        user_id: {
            type: Schema.Types.ObjectId,
            ref: 'User',
            required: true,
        },
        title: {
            type: String,
            required: true,
        },
        notes: {
            type: String,
            required: false,
        },
        start: {
            type: Date,
            required: true,
        },
        end: {
            type: Date,
            required: true,
        },
        duration_ms: {
            type: Number,
            required: true,
            alias: 'durationMs',
        },
        exercises: [
            {
                type: Schema.Types.ObjectId,
                ref: 'Exercise',
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

export type {WorkoutDocument, WorkoutProperties};
export default model<WorkoutProperties>('Workout', WorkoutSchema);
