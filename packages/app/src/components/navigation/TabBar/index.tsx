import {type Component, type ComponentProps, For} from 'solid-js';

import TabButton from '@/components/navigation/TabButton';

import styles from './styles.module.scss';

const TabBar: Component<{
    links: ComponentProps<typeof TabButton>[];
}> = props => {
    return (
        <div class="flex">
            <nav class={styles.nav}>
                <For each={props.links}>
                    {link => (
                        <TabButton
                            icon={link.icon}
                            label={link.label}
                            route={link.route}
                        />
                    )}
                </For>
            </nav>
        </div>
    );
};

export default TabBar;