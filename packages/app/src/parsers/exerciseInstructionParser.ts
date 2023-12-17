import {number, object, type output, string} from 'zod';

export const exerciseInstructionSchema = object({
    sequence_number: number(),
    content: string(),
});

export type ExerciseInstruction = output<typeof exerciseInstructionSchema>;
