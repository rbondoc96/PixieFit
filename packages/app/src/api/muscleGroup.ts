import {compile} from 'path-to-regexp';

import {client} from '@/api/client';
import {type MuscleGroup, muscleGroupListParser} from '@/parsers/muscleGroupParser';
import delay from '@/utilities/delay';

const muscleGroupListRoute = compile('/api/muscle-groups');

export async function listMuscleGroups(): Promise<MuscleGroup[]> {
    const {data} = await client.get(muscleGroupListRoute());

    await delay(1000);

    return muscleGroupListParser.parse(data).data;
}
