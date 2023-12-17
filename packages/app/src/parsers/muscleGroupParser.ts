import {nullable, number, object, type output, string} from 'zod';

import {createListResponseParser} from '@/parsers/responseParsers';

export const muscleGroupSchema = object({
    id: number(),
    name: string(),
    image_source: nullable(string()),
});

export const muscleGroupListParser = createListResponseParser(muscleGroupSchema);
export type MuscleGroup = output<typeof muscleGroupSchema>;
