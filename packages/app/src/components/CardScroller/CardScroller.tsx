import {type Component, For} from 'solid-js';

import Card, {type CardProps} from '@/components/CardScroller/Card';

import styles from './styles.module.scss';

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
