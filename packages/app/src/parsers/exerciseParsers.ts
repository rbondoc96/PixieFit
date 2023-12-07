import {array, nativeEnum, nullable, object, type output, string} from 'zod';

import ExerciseForce from '@/enums/ExerciseForce';
import ExerciseMechanic from '@/enums/ExerciseMechanic';
import ExerciseType from '@/enums/ExerciseType';
import {exerciseEquipmentSchema} from '@/parsers/exerciseEquipmentParser';
import {measurementSchema} from '@/parsers/measurementParser';
import {simplifiedMuscleSchema} from '@/parsers/muscleParser';
import {muscleGroupSchema} from '@/parsers/muscleGroupParser';
import {createGetResponseParser, createListResponseParser} from '@/parsers/responseParsers';

const simplifiedExerciseSchema = object({
    id: string(),
    type: nativeEnum(ExerciseType),
    target_muscle_group: nullable(muscleGroupSchema),
    name: string(),
    name_alternative: nullable(string()),
    description: nullable(string()),
    equipment: nullable(exerciseEquipmentSchema),
    mechanic: nullable(nativeEnum(ExerciseMechanic)),
    force: nullable(nativeEnum(ExerciseForce)),
    measurement: nullable(measurementSchema),
});

const exerciseSchema = simplifiedExerciseSchema.extend({
    primary_muscles: array(simplifiedMuscleSchema),
    secondary_muscles: array(simplifiedMuscleSchema),
    tertiary_muscles: array(simplifiedMuscleSchema),
});

export const exerciseListParser = createListResponseParser(simplifiedExerciseSchema);
export const exerciseReadParser = createGetResponseParser(exerciseSchema);

export type SimplifiedExercise = output<typeof simplifiedExerciseSchema>;
export type Exercise = output<typeof exerciseSchema>;
