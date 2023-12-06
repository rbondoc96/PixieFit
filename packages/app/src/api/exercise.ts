import {compile} from 'path-to-regexp';

import {client} from '@/api/client';
import {
    exerciseListParser,
    type SimplifiedExercise,
} from '@/parsers/exerciseParsers';

const exerciseListRoute = compile('/api/exercises');

export type ListMusclesQueryParams = {
    muscle_group?: string;
};

export async function listMuscles(
    params: ListMusclesQueryParams = {},
): Promise<SimplifiedExercise[]> {
    const queryParams = new URLSearchParams();

    if (params.muscle_group) {
        queryParams.append('muscle_group', params.muscle_group);
    }

    const {data} = await client.get(`${exerciseListRoute()}?${queryParams.toString()}`);

    return exerciseListParser.parse(data).data;
}
