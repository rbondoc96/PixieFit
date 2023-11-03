import {type IconDefinition} from '@fortawesome/free-solid-svg-icons';
import {A as Link} from '@solidjs/router';
import FontAwesomeIcon from 'solid-fa';
import {type Component, type JSX, Show} from 'solid-js';

import useRouter from '@/hooks/useRouter';
import type {ClickEventHandler} from '@/lib/types';

const RouterLink: Component<{
    'class'?: string;
    label: string;
    href: string;
    icon?: IconDefinition;
    onClick?: ClickEventHandler<HTMLAnchorElement>;
}> = props => {
    const router = useRouter();

    const handleClick: JSX.EventHandler<HTMLAnchorElement, MouseEvent> = event => {
        event.preventDefault();
        event.stopPropagation();

        if (props.onClick) {
            props.onClick(event);
        }

        router.push(props.href);
    };

    return (
        <Link
            href={props.href}
            class={props.class}
            onClick={handleClick}
        >
            <div class="flex gap-x-2 items-center">
                <Show when={props.icon} keyed>
                    {icon => (
                        <FontAwesomeIcon
                            icon={icon}
                        />
                    )}
                </Show>
                <span>
                    {props.label}
                </span>
            </div>
        </Link>
    );
};

export default RouterLink;
