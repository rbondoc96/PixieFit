import {Match, mergeProps, type ParentComponent, Switch} from 'solid-js';

import styles from './styles.module.scss';

type HTMLTextTag = 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6' | 'p';

type TextProps = {
    as?: HTMLTextTag;
    size?: string;
};

const Text: ParentComponent<TextProps> = baseProps => {
    const props = mergeProps(
        {
            as: 'p',
        },
        baseProps,
    );

    return (
        <Switch>
            <Match when={props.as === 'h1'}>
                <h1 class={styles.text}>{props.children}</h1>
            </Match>
            <Match when={props.as === 'h2'}>
                <h2 class={styles.text}>{props.children}</h2>
            </Match>
            <Match when={props.as === 'h3'}>
                <h3 class={styles.text}>{props.children}</h3>
            </Match>
            <Match when={props.as === 'h4'}>
                <h4 class={styles.text}>{props.children}</h4>
            </Match>
            <Match when={props.as === 'h5'}>
                <h5 class={styles.text}>{props.children}</h5>
            </Match>
            <Match when={props.as === 'h6'}>
                <h6 class={styles.text}>{props.children}</h6>
            </Match>
            <Match when={props.as === 'p'}>
                <p class={styles.text}>{props.children}</p>
            </Match>
        </Switch>
    );
};

export default Text;
