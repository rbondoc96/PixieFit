import {A as Link} from '@solidjs/router';
import clsx from 'clsx';
import {type Component} from 'solid-js';

import styles from './styles.module.scss';

type RouterLinkProps = {
    className?: string;
    label: string;
    href: string;
};

const RouterLink: Component<RouterLinkProps> = ({className, label, href}) => {
    return (
        <Link href={href} class={clsx(styles.routerLink, className)}>
            {label}
        </Link>
    );
};

export default RouterLink;
