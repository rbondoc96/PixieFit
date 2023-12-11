import {useParams} from '@solidjs/router';
import {type Component, createResource, For, Show} from 'solid-js';

import {ExerciseAPI} from '@/api';
import {displayExerciseForce} from '@/enums/ExerciseForce';
import {displayExerciseType} from '@/enums/ExerciseType';
import {useUser} from '@/stores/auth.store';

type PageParams = {
    id: string;
};

const ExerciseDetailsPage: Component = () => {
    const params = useParams<PageParams>();
    const user = useUser();

    const [exercise] = createResource(
        () => user(),
        async () => await ExerciseAPI.readMuscle(params.id),
    );

    return (
        <section class="flex flex-col gap-y-4 px-6 py-6">
            <Show when={exercise()} keyed>
                {details => (
                    <>
                        <span class="text-3xl font-extrabold tracking-tight">
                            {details.name}
                        </span>
                        <span>
                            Type: {displayExerciseType(details.type)}
                        </span>
                        <Show when={details.target_muscle_group} keyed>
                            {group => (
                                <div>
                                    Target Muscle Group: {group.name}
                                </div>
                            )}
                        </Show>
                        <Show when={details.equipment} keyed>
                            {equipment => (
                                <div>
                                    Equipment: {equipment.name}
                                </div>
                            )}
                        </Show>
                        <Show when={details.force} keyed>
                            {force => (
                                <div>
                                    Force: {displayExerciseForce(force)}
                                </div>
                            )}
                        </Show>
                        <For each={details.primary_muscles}>
                            {muscle => (
                                <div>
                                    <span>{muscle.name}</span>
                                </div>
                            )}
                        </For>
                        <For each={details.secondary_muscles}>
                            {muscle => (
                                <div>
                                    <span>{muscle.name}</span>
                                    <Show when={muscle.image_source} keyed>
                                        {source => (
                                            <img alt={muscle.name} src={source} />
                                        )}
                                    </Show>
                                </div>
                            )}
                        </For>
                        <For each={details.tertiary_muscles}>
                            {muscle => (
                                <div>
                                    <span>{muscle.name}</span>
                                    <Show when={muscle.image_source} keyed>
                                        {source => (
                                            <img alt={muscle.name} src={source} />
                                        )}
                                    </Show>
                                </div>
                            )}
                        </For>
                        <Show when={details.instructions.length > 0}>
                            <span class="text-xl font-medium">
                                Instructions
                            </span>
                        </Show>
                        <For each={details.instructions}>
                            {instruction => (
                                <div class="flex gap-x-2">
                                    <span>{instruction.sequence_number}.</span>
                                    <span>{instruction.content}</span>
                                </div>
                            )}
                        </For>
                    </>
                )}
            </Show>
        </section>
    );
};

export default ExerciseDetailsPage;
