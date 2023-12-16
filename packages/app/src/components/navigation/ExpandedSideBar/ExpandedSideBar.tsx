import {type Component, createMemo, Show} from 'solid-js';

import Transition from '@/components/Transition';

const ExpandedSideBar: Component<{
    showSidebar: boolean;
    version?: [number, number, number, 'dev' | 'rc'],
    onClose: () => void;
}> = props => {
    const version = createMemo(
        () => {
            if (!props.version) {
                return undefined;
            }

            const [major, minor, patch, stage] = props.version;

            return `v${major}.${minor}.${patch}${stage ? `-${stage}` : ''}`;
        },
    );

    return (
        <aside>
            <Transition
                show={props.showSidebar}
                enter="duration-300"
                enterFrom="opacity-0"
                enterTo="opacity-100"
                leave="duration-300"
                leaveFrom="opacity-100"
                leaveTo="opacity-0"
            >
                <div
                    role="presentation"
                    class="fixed inset-0 z-navOverlay bg-black/50"
                    onClick={() => props.onClose()}
                />
            </Transition>
            <Transition
                show={props.showSidebar}
                enter="duration-300"
                enterFrom="-translate-x-full"
                enterTo="translate-x-0"
                leave="duration-300"
                leaveFrom="translate-x-0"
                leaveTo="-translate-x-full"
            >
                <div class="fixed left-0 h-screen w-72 z-navContent">
                    <div class="h-full bg-gray-900">
                        <div class="flex flex-col">
                            <span>
                                ExpandedSideBar
                            </span>

                            <Show when={version()}>
                                {ver => (
                                    <span>
                                        {ver()}
                                    </span>
                                )}
                            </Show>
                        </div>
                    </div>
                </div>
            </Transition>
        </aside>
    );
};

export default ExpandedSideBar;
