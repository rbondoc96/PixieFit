import {A as Link} from '@solidjs/router';
import {type Component, createMemo, Match, mergeProps, Show, splitProps, Switch} from 'solid-js';

import Spinner from '@/components/Spinner';
import matches from '@/utilities/matches';

import styles from './styles.module.scss';

type HTMLAnchorProps = {
    as: 'a';
    href: string;
    rel?: 'noopener noreferrer' | 'noreferrer' | 'noopener';
    target?: '_blank';
};

type HTMLButtonProps = {
    as?: 'button';
    type?: 'button' | 'submit';
    onClick?: () => void;
};

type LinkProps = {
    as: typeof Link;
    href: string;
};

type ButtonVariant = 'primary' | 'secondary';

type ButtonProps = {
    disabled?: boolean;
    isLoading?: boolean;
    label: string;
    variant?: ButtonVariant;
} & (HTMLAnchorProps | HTMLButtonProps | LinkProps);

function isHTMLAnchorElement(props: unknown): props is HTMLAnchorProps {
    return (props as HTMLAnchorProps).as === 'a';
}

function isHTMLButtonElement(props: unknown): props is HTMLButtonProps {
    return (
        (props as HTMLButtonProps).as === 'button' || (props as HTMLButtonProps).as === undefined
    );
}

function isRouterLinkElement(props: unknown): props is LinkProps {
    return (props as LinkProps).as === Link;
}

const Button: Component<ButtonProps> = props => {
    const propsWithDefaults = mergeProps(
        {
            as: 'button',
            disabled: false,
            isLoading: false,
            variant: 'primary',
        },
        props,
    );

    const [split, rest] = splitProps(propsWithDefaults, [
        'variant',
        'disabled',
        'isLoading',
        'label',
    ]);

    const shouldDisable = createMemo(() => split.disabled || split.isLoading);

    const classList = createMemo(() => ({
        [styles.button]: true,
        [styles.buttonPrimary]: split.variant === 'primary',
        [styles.buttonSecondary]: split.variant === 'secondary',
        [styles.buttonLoading]: split.isLoading,
    }));

    return (
        <Switch>
            <Match when={matches(rest, isHTMLAnchorElement)}>
                {item => (
                    <a href={item().href} aria-disabled={shouldDisable()} classList={classList()}>
                        {split.label}
                    </a>
                )}
            </Match>
            <Match when={matches(rest, isHTMLButtonElement)}>
                {item => (
                    <button
                        disabled={shouldDisable()}
                        type={item().type}
                        aria-disabled={shouldDisable()}
                        classList={classList()}
                        onClick={item().onClick}
                    >
                        <Show when={split.isLoading}>
                            <Spinner />
                        </Show>
                        {split.label}
                    </button>
                )}
            </Match>
            <Match when={matches(rest, isRouterLinkElement)}>
                {item => (
                    <Link
                        href={item().href}
                        aria-disabled={shouldDisable()}
                        classList={classList()}
                    >
                        {split.label}
                    </Link>
                )}
            </Match>
        </Switch>
    );
};

export default Button;
