import {type Component, createMemo} from 'solid-js';

import CardScroller, {type CardProps} from '@/components/CardScroller';
import {useMuscleGroupList, useMuscleList} from '@/stores/muscle.store';

const ExercisesPage: Component = () => {
    const muscles = useMuscleList();
    const muscleGroups = useMuscleGroupList();

    const musclesAsItems = createMemo<CardProps[]>(() => muscles()
        .map(muscle => ({
            href: `/app/muscles/${muscle.id}`,
            image: {
                alt: muscle.simple_name ?? muscle.name,
                src: muscle.image_source ?? undefined,
            },
            title: muscle.simple_name ?? muscle.name,
        })),
    );

    const muscleGroupsAsItems = createMemo<CardProps[]>(() => muscleGroups()
        .map(group => ({
            href: `/app/muscle-groups/${group.id}`,
            image: {
                alt: group.name,
                src: group.image_source ?? undefined,
            },
            title: group.name,
        })),
    );

    return (
        <section class="flex flex-col px-4">
            <h1>Exercises Page</h1>

            <div class="relative">
                <CardScroller
                    title="Exercises by Muscle Group"
                    items={muscleGroupsAsItems()}
                />
                <CardScroller
                    title="Exercises by Muscle"
                    items={musclesAsItems()}
                />
            </div>
        </section>
    );
};

export default ExercisesPage;
