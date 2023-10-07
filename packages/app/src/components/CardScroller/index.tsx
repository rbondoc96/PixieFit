import {A as Link} from '@solidjs/router';
import {type Component, For, Show} from 'solid-js';

import styles from './styles.module.scss';

export type CardProps = {
    href: string;
    image?: {
        alt: string;
        'class'?: string;
        src: string;
    };
    title: string;
};

const Card: Component<CardProps> = props => {
    return (
        <Link
            href={props.href}
            class={styles.card}
        >
            <div class={styles.cardOverlay} />
            <div class={styles.cardContent}>
                <div class={styles.cardContentContainer}>
                    <span class={styles.cardContentTitle}>
                        {props.title}
                    </span>
                </div>
            </div>
            <div class={styles.cardBackground}>
                <Show
                    keyed
                    when={props.image}
                >
                    {image => (
                        <img
                            class={image.class}
                            src={image.src}
                            alt={image.alt}
                        />
                    )}
                </Show>
            </div>
        </Link>
    );
};

const CardScroller: Component<{
    items: CardProps[];
    title: string;
}> = props => {
    return (
        <div class={styles.cardScroller}>
            <div class={styles.cardScrollerHeading}>
                <h2>
                    {props.title}
                </h2>
            </div>
            <div class={styles.cardScrollerContent}>
                <div class={styles.cardScrollerContentCards}>
                    <For each={props.items}>
                        {item => <Card {...item} />}
                    </For>
                </div>
            </div>
        </div>
    );
};

export default CardScroller;
