import {createResource, type InitializedResource} from 'solid-js';

import {MuscleAPI, MuscleGroupAPI} from '@/api';
import {type MuscleGroup} from '@/parsers/muscleGroupParser';
import {type SimplifiedMuscle} from '@/parsers/muscleParser';
import {useUser} from '@/stores/auth.store';

const user = useUser();

const [muscleListResource] = createResource(
    () => user(),
    MuscleAPI.listMuscles,
    {
        initialValue: [],
    },
);

const [muscleGroupListResource] = createResource(
    () => user(),
    MuscleGroupAPI.listMuscleGroups,
    {
        initialValue: [],
    },
);

export const useMuscleList = (): InitializedResource<SimplifiedMuscle[]> => muscleListResource;
export const useMuscleGroupList = (): InitializedResource<MuscleGroup[]> => muscleGroupListResource;

