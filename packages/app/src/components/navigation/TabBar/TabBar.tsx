import {type Component, type ComponentProps, For} from 'solid-js';

import TabButton from '@/components/navigation/TabBar/TabButton';

const TabBar: Component<{
    links: ComponentProps<typeof TabButton>[];
}> = props => {
    return (
        <div class="flex">
            <nav class="flex-1 flex h-14 bg-gray-900 p-2">
                <For each={props.links}>
                    {link => (
                        <TabButton
                            class="flex-1"
                            icon={link.icon}
                            route={link.route}
                        />
                    )}
                </For>
            </nav>
        </div>
    );
};

export default TabBar;