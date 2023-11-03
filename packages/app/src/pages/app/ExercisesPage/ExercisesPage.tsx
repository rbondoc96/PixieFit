import {type Component, createMemo} from 'solid-js';

import CardScroller, {type CardProps} from '@/components/CardScroller';
import {useMuscleGroupList} from '@/stores/muscle.store';

const ExercisesPage: Component = () => {
    const muscleGroups = useMuscleGroupList();

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
        <section class="flex flex-col gap-y-4 pl-6 py-6">
            <h1 class="text-3xl font-extrabold tracking-tight">
                Exercises Page
            </h1>

            <div class="flex flex-col gap-y-8 relative">
                <CardScroller
                    title="Exercises by Muscle Group"
                    items={muscleGroupsAsItems()}
                />
            </div>
        </section>
    );
};

export default ExercisesPage;
