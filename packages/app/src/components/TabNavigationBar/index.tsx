import {
    faChartColumn,
    faDumbbell,
    faGear,
    faList,
    faStopwatch,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component} from 'solid-js';

import {
    Exercises,
    UserDashboard,
    UserProgress,
    UserSettings,
    UserTracker,
} from '@/constants/Routes';
import type {Route} from '@/lib/Route';

import styles from './styles.module.scss';

type Tab = {
    icon: IconDefinition;
    label: string;
    route: Route;
};

const TabButton: Component<Tab> = props => {
    return (
        <Link
            href={props.route.fullPath}
            activeClass={styles.tabButtonActive}
            class={styles.tabButton}
        >
            <FontAwesomeIcon icon={props.icon} size="lg" />
            <span>{props.label}</span>
        </Link>
    );
};

const TabNavigationBar: Component = () => {
    return (
        <nav class={styles.nav}>
            <TabButton icon={faList} label="Dashboard" route={UserDashboard} />
            <TabButton icon={faDumbbell} label="Exercises" route={Exercises} />
            <Link
                href={UserTracker.fullPath}
                activeClass={styles.middleTabActive}
                class={styles.middleTab}
            >
                <div class={styles.middleTabButton}>
                    <FontAwesomeIcon icon={faStopwatch} size="1x" />
                </div>
                <span class={styles.middleTabLabel}>Tracker</span>
            </Link>
            <TabButton icon={faChartColumn} label="Progress" route={UserProgress} />
            <TabButton icon={faGear} label="Settings" route={UserSettings} />
        </nav>
    );
};

export default TabNavigationBar;
