import {compile} from 'path-to-regexp';

import {client} from '@/api/client';
import {
    type Muscle,
    muscleListParser,
    muscleReadParser,
    type SimplifiedMuscle,
} from '@/parsers/muscleParser';

const muscleListRoute = compile('/api/muscles');
const muscleReadRoute = compile<{
    id: number;
}>('/api/muscles/:id');

export async function listMuscles(): Promise<SimplifiedMuscle[]> {
    const {data} = await client.get(muscleListRoute());

    return muscleListParser.parse(data).data;
}

export async function readMuscle(id: number): Promise<Muscle> {
    const {data} = await client.get(muscleReadRoute({id}));

    return muscleReadParser.parse(data).data;
}
