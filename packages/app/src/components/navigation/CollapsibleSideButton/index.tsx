import {type IconDefinition} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component} from 'solid-js';

import Transition from '@/components/Transition';
import {Route} from '@/lib/Route';

import styles from './styles.module.scss';

const CollapsibleSideButton: Component<{
    icon: IconDefinition;
    isExpanded?: boolean;
    label: string;
    routeOrHref: Route | string;
    rel?: 'noopener noreferrer' | 'noreferrer' | 'noopener';
    target?: '_blank';
}> = props => (
    <Link
        href={props.routeOrHref instanceof Route ? props.routeOrHref.href : props.routeOrHref}
        activeClass={styles.collapsibleSideButtonActive}
        class={styles.collapsibleSideButton}
        rel={props.rel}
        target={props.target}
    >
        <FontAwesomeIcon
            class={styles.collapsibleSideButtonIcon}
            icon={props.icon}
        />
        <Transition
            show={props.isExpanded}
            enter={styles.collapsibleSideButtonLabel}
            enterFrom={styles.collapsibleSideButtonLabelInvisible}
            enterTo={styles.collapsibleSideButtonLabelVisible}
            leave={styles.collapsibleSideButtonLabel}
            leaveFrom={styles.collapsibleSideButtonLabelVisible}
            leaveTo={styles.collapsibleSideButtonLabelInvisible}
        >
            <span class="text-sm">
                {props.label}
            </span>
        </Transition>
    </Link>
);

export default CollapsibleSideButton;
