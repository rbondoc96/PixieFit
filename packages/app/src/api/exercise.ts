import {compile} from 'path-to-regexp';

import {client} from '@/api/client';
import {
    type Exercise,
    exerciseListParser,
    exerciseReadParser,
    type SimplifiedExercise,
} from '@/parsers/exerciseParsers';

const exerciseListRoute = compile('/api/exercises');
const exerciseReadRoute = compile<{
    id: string;
}>('/api/exercises/:id');

export type ListMusclesQueryParams = {
    muscle_group?: string;
    page?: string;
    per_page?: string;
};

export async function listMuscles(
    params: ListMusclesQueryParams = {},
): Promise<SimplifiedExercise[]> {
    const queryParams = new URLSearchParams({
        page: params.page ?? '1',
        per_page: params.per_page ?? '10',
    });

    if (params.muscle_group) {
        queryParams.append('muscle_group', params.muscle_group);
    }

    const {data} = await client.get(`${exerciseListRoute()}?${queryParams.toString()}`);

    return exerciseListParser.parse(data).data;
}

export async function readMuscle(
    id: string,
): Promise<Exercise> {
    const {data} = await client.get(exerciseReadRoute({id}));

    return exerciseReadParser.parse(data).data;
}
