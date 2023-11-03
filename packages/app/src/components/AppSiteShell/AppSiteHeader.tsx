import {faChevronDown, faSignOut} from '@fortawesome/free-solid-svg-icons';
import FontAwesomeIcon from 'solid-fa';
import {type Component, createEffect, createSignal, Show} from 'solid-js';

import UserIcon from '@/assets/images/user.png';
import RouterLink from '@/components/RouterLink';
import {Logout} from '@/constants/Routes';
import {logout} from '@/stores/auth.store';

const AppSiteHeader: Component = () => {
    let activeArea: HTMLDivElement | undefined;

    const [isMenuDisplayed, setIsMenuDisplayed] = createSignal(false);

    const toggleMenu = () => setIsMenuDisplayed(state => !state);

    const outsideClickDetector = (event: MouseEvent) => {
        if (!activeArea?.contains(event.target as Node)) {
            setIsMenuDisplayed(false);
        }
    };

    createEffect(() => {
        if (isMenuDisplayed()) {
            document.addEventListener('click', outsideClickDetector);
        } else {
            document.removeEventListener('click', outsideClickDetector);
        }
    });

    return (
        <div class="flex-1 flex items-center justify-end">
            <div class="relative" ref={activeArea}>
                <button
                    type="button"
                    class="flex gap-x-2 items-center justify-center focus:ring focus:ring-blue-500"
                    onClick={toggleMenu}
                >
                    <img
                        class="w-[35px] h-auto aspect-square"
                        src={UserIcon}
                        alt="user icon"
                    />
                    <FontAwesomeIcon
                        icon={faChevronDown}
                    />
                </button>
                <Show when={isMenuDisplayed()}>
                    <div class="absolute right-1/4 z-10">
                        <div class="flex">
                            <div class="flex-1 w-36 bg-gray-100 rounded-sm drop-shadow-md">
                                <ul class="mx-5 my-3">
                                    <li class="flex">
                                        <RouterLink
                                            class="font-medium text-xs hover:text-indigo-600"
                                            href={Logout.href}
                                            icon={faSignOut}
                                            label="Sign Out"
                                            onClick={logout}
                                        />
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    );
};

export default AppSiteHeader;





