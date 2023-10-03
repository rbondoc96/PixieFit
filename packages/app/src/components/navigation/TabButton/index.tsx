import {type IconDefinition} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component} from 'solid-js';

import {type Route} from '@/lib/Route';

import styles from './styles.module.scss';

const TabButton: Component<{
    icon: IconDefinition;
    label: string;
    route: Route;
}> = props => {
    return (
        <Link
            href={props.route.href}
            activeClass={styles.tabButtonActive}
            class={styles.tabButton}
        >
            <FontAwesomeIcon icon={props.icon} size="1x" />
            <span>{props.label}</span>
        </Link>
    );
};

export default TabButton;
