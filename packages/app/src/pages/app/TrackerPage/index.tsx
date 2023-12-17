import {A as Link} from '@solidjs/router';
import {type Component} from 'solid-js';

import Button from '@/components/Button';

import styles from './styles.module.scss';

const TrackerPage: Component = () => {
    return (
        <section class={styles.section}>
            <h1>Some Title</h1>
            <div class={styles.buttons}>
                <Button
                    as={Link}
                    class={styles.button}
                    label="Add a One-off Exercise"
                    href="/app/tracker/ayg"
                />
                <Button
                    as={Link}
                    class={styles.button}
                    label="Start a Workout"
                    href="/app/tracker/ayg"
                />
            </div>
            <div class={styles.workouts}>
                <div class={styles.workout}>
                    <h2>Today&apos;s Workout</h2>
                    <div>
                        <span>No workouts...yet!</span>
                    </div>
                </div>
                <div class={styles.workout}>
                    <h2>One-off Exercises</h2>
                    <div class={styles.exerciseList}>
                        <span>No workouts...yet!</span>
                    </div>
                </div>
            </div>
        </section>
    );
};

export default TrackerPage;
