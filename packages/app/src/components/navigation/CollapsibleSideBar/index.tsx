import {
    faChevronLeft,
    faChevronRight,
    faCircleQuestion,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {type Component, createMemo, For, Show} from 'solid-js';

import Logo from '@/components/Logo';
import CollapsibleSideButton from '@/components/navigation/CollapsibleSideButton';
import {type Route} from '@/lib/Route';

const CollapsibleSideBar: Component<{
    isExpanded: boolean;
    links: Array<{
        icon: IconDefinition;
        isExpanded?: boolean;
        label: string;
        route: Route;
    }>;
    onToggle: () => void;
    version?: {
        major: number;
        minor: number;
        patch: number;
        stage?: 'dev' | 'rc';
    }
}> = props => {
    const version = createMemo(
        () => props.version
            ? `v${props.version.major}.${props.version.minor}.${props.version.patch}${
                props.version.stage ? `-${props.version.stage}` : ''
            }`
            : undefined,
    );

    return (
        <div
            classList={{
                'flex flex-col justify-between bg-gray-900 h-screen relative duration-500': true,
                'w-72': props.isExpanded,
                'w-20': !props.isExpanded,
            }}
        >
            <div class="flex flex-col gap-y-12 p-5 pt-8">
                <div class="absolute top-9 left-full -translate-x-1/2">
                    <div class="flex rounded-full border border-black w-6 h-6 bg-white">
                        <button
                            class="flex-1 flex items-center justify-center"
                            type="button"
                            onClick={() => props.onToggle()}
                        >
                            <FontAwesomeIcon
                                icon={props.isExpanded ? faChevronLeft : faChevronRight}
                                size="xs"
                            />
                        </button>
                    </div>
                </div>
                <div class="flex items-center">
                    <Logo
                        showText={props.isExpanded}
                        tagLabel="dev"
                        theme="dark"
                    />
                </div>
                <nav>
                    <ul class="flex flex-col gap-y-2">
                        <For each={props.links}>
                            {link => (
                                <li>
                                    <CollapsibleSideButton
                                        isExpanded={props.isExpanded}
                                        icon={link.icon}
                                        label={link.label}
                                        routeOrHref={link.route}
                                    />
                                </li>
                            )}
                        </For>
                    </ul>
                </nav>
            </div>
            <div>
                <div class="px-5">
                    <ul class="flex flex-col gap-y-2">
                        <li>
                            <CollapsibleSideButton
                                isExpanded={props.isExpanded}
                                icon={faCircleQuestion}
                                label="Feedback"
                                routeOrHref="https://github.com"
                            />
                        </li>
                    </ul>
                </div>
                <Show when={version()} keyed>
                    {version => (
                        <div class="flex justify-center py-2">
                            <span class="text-primary text-[10px]">
                                {version}
                            </span>
                        </div>
                    )}
                </Show>
            </div>
        </div>
    );
};

export default CollapsibleSideBar;