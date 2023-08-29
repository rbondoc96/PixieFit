import {createClient} from '@/services/client';
import {type Muscle, muscleListParser, muscleParser} from '@/services/WGER/parsers';

const client = createClient({
    baseURL: env('WGER_API_URL'),
    timeout: 10000,
});

export async function listMuscles(): Promise<Muscle[]> {
    const {data} = await client.get('/muscle');

    return muscleListParser.parse(data).results;
}

export async function readMuscle(id: Muscle['id']): Promise<Muscle> {
    const {data} = await client.get(`/muscle/${id}`);

    return muscleParser.parse(data);
}
