import {number, object, type output, string} from 'zod';

export const exerciseEquipmentSchema = object({
    id: number(),
    name: string(),
});

export type ExerciseEquipment = output<typeof exerciseEquipmentSchema>;
