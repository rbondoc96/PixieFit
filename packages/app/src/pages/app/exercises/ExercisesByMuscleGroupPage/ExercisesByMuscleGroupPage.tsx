import {useParams} from '@solidjs/router';
import {type Component, createResource, For} from 'solid-js';

import {ExerciseAPI} from '@/api';
import {useUser} from '@/stores/auth.store';

type PageParams = {
    id: string;
};

const ExercisesByMuscleGroupPage: Component = () => {
    const params = useParams<PageParams>();
    const user = useUser();

    const [exercises] = createResource(
        () => user(),
        async () => await ExerciseAPI.listMuscles({muscle_group: params.id}),
        {
            initialValue: [],
        },
    );

    return (
        <section class="flex flex-col gap-y-4 px-6 py-6">
            <span class="text-3xl font-extrabold tracking-tight">
                Exercises by Muscle Group
            </span>
            <For each={exercises()}>
                {exercise => (
                    <div class="border border-black rounded-md px-2 py-2">
                        <div class="flex flex-col">
                            <span>
                                {exercise.name}
                            </span>
                        </div>
                    </div>
                )}
            </For>
        </section>
    );
};

export default ExercisesByMuscleGroupPage;
