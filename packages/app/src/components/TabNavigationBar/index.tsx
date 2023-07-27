import {
    faChartSimple,
    faDumbbell,
    faGears,
    faStopwatch,
    faUserNinja,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component, Index} from 'solid-js';

import {
    Exercises,
    UserDashboard,
    UserProgress,
    UserSettings,
    UserWorkoutTracker,
} from '@/constants/Routes';
import type {Route} from '@/lib/Route';

import styles from './styles.module.scss';

type Tab = {
    icon: IconDefinition;
    label: string;
    route: Route;
};

const tabs: Tab[] = [
    {
        icon: faUserNinja,
        label: 'Home',
        route: UserDashboard,
    },
    {
        icon: faDumbbell,
        label: 'Exercises',
        route: Exercises,
    },
    {
        icon: faStopwatch,
        label: 'Tracker',
        route: UserWorkoutTracker,
    },
    {
        icon: faChartSimple,
        label: 'Progress',
        route: UserProgress,
    },
    {
        icon: faGears,
        label: 'Settings',
        route: UserSettings,
    },
];

const TabNavigationBar: Component = () => {
    return (
        <nav class={styles.nav}>
            <Index each={tabs}>
                {tab => (
                    <Link
                        href={tab().route.fullPath}
                        activeClass={styles.tabButtonActive}
                        class={styles.tabButton}
                    >
                        <FontAwesomeIcon icon={tab().icon} />
                        <span>{tab().label}</span>
                    </Link>
                )}
            </Index>
        </nav>
    );
};

export default TabNavigationBar;
