import {nullable, object, type output, string} from 'zod';

import {createGetResponseParser, createListResponseParser} from '@/parsers/responseParsers';

const simplifiedMuscleSchema = object({
    id: string(),
    muscle_group: string(),
    name: string(),
    simple_name: nullable(string()),
    description: nullable(string()),
    image_source: nullable(string()),
});

const muscleSchema = object({
    id: string(),
    muscle_group: string(),
    name: string(),
    simple_name: nullable(string()),
    description: nullable(string()),
    image_source: nullable(string()),
    parent: nullable(simplifiedMuscleSchema),
});

export const muscleListParser = createListResponseParser(simplifiedMuscleSchema);
export const muscleReadParser = createGetResponseParser(muscleSchema);

export type SimplifiedMuscle = output<typeof simplifiedMuscleSchema>;
export type Muscle = output<typeof muscleSchema>;
