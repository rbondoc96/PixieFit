import {A as Link, useParams, useSearchParams} from '@solidjs/router';
import {type Component, createResource, For, onMount, Show} from 'solid-js';

import {ExerciseAPI} from '@/api';

type PageParams = {
    id: string;
};

type SearchParams = {
    page: string;
    per_page: string;
};

const ExercisesByMuscleGroupPage: Component = () => {
    const params = useParams<PageParams>();
    const [searchParams, setSearchParams] = useSearchParams<SearchParams>();

    onMount(() => {
        setSearchParams({
            page: '1',
            per_page: '10',
        });
    });

    const [exercises] = createResource(
        () => ({
            muscle_group: params.id,
            page: searchParams.page,
            per_page: searchParams.per_page,
        }),
        ExerciseAPI.listMuscles,
        {
            initialValue: [],
        },
    );

    return (
        <section class="flex flex-col gap-y-4 px-6 py-6">
            <span class="text-3xl font-extrabold tracking-tight">
                Exercises by Muscle Group
            </span>
            <div class="flex flex-wrap gap-x-2 gap-y-2">
                <For each={exercises()}>
                    {exercise => (
                        <Link href={`/app/exercises/${exercise.id}`}>
                            <div class="w-36 h-36 border border-black rounded-md px-2 py-2">
                                <div class="flex flex-col">
                                    <div class="flex flex-col-reverse gap-y-2">
                                        <span>
                                            {exercise.name}
                                        </span>
                                        <Show when={exercise.primary_muscles.length > 0}>
                                            <span class="text-sm">
                                                {exercise.primary_muscles.map(muscle => muscle.name).join(', ')}
                                            </span>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        </Link>
                    )}
                </For>
            </div>
            <div class="flex">
                <Show when={Number(searchParams.page) > 1}>
                    <button
                        class="flex-1 border border-black rounded-md px-2 py-2"
                        onClick={() => setSearchParams({
                            page: (Number(searchParams.page) - 1).toString(),
                            per_page: searchParams.per_page,
                        })}
                    >
                        Previous Page
                    </button>
                </Show>
                <button
                    class="flex-1 border border-black rounded-md px-2 py-2"
                    onClick={() => setSearchParams({
                        page: (Number(searchParams.page) + 1).toString(),
                        per_page: searchParams.per_page,
                    })}
                >
                    Next Page
                </button>
            </div>
        </section>
    );
};

export default ExercisesByMuscleGroupPage;
