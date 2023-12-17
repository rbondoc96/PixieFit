import {A as Link} from '@solidjs/router';
import {type Component, Show} from 'solid-js';

import styles from './styles.module.scss';

export type CardProps = {
    href: string;
    image: {
        alt?: string;
        'class'?: string;
        src?: string;
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
                    when={props.image.src}
                    fallback={<div class="bg-blue-800 flex-1 rounded-lg" />}
                >
                    {src => (
                        <img
                            class={props.image.class}
                            src={src}
                            alt={props.image.alt}
                        />
                    )}
                </Show>
            </div>
        </Link>
    );
};

export default Card;
