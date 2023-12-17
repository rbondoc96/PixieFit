import {type IconDefinition} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component} from 'solid-js';

import {type Route} from '@/lib/Route';

import styles from './styles.module.scss';

const TabButton: Component<{
    'class'?: string;
    icon: IconDefinition;
    route: Route;
}> = props => {
    return (
        <Link
            href={props.route.href}
            activeClass={styles.tabButtonActive}
            classList={{
                [styles.tabButton]: true,
                [props.class ?? '']: true,
            }}
        >
            <FontAwesomeIcon icon={props.icon} size="lg" />
        </Link>
    );
};

export default TabButton;
