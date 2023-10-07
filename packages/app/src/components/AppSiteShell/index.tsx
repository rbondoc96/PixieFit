import {
    faDumbbell,
    faHome,
    faList,
    faStopwatch,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import {Outlet} from '@solidjs/router';
import {type Component, createSignal} from 'solid-js';

import UserIcon from '@/assets/images/user.png';
import AuthenticatedView from '@/components/AuthenticatedView';
import Helmet from '@/components/Helmet';
import LoadingView from '@/components/LoadingView';
import CollapsibleSideBar from '@/components/navigation/CollapsibleSideBar';
import SuspensefulErrorBoundary from '@/components/SuspensefulErrorBoundary';
import TabBar from '@/components/navigation/TabBar';
import {
    Exercises,
    UserDashboard,
    UserProgress,
    UserTracker,
} from '@/constants/Routes';
import {type Route} from '@/lib/Route';
import GeneralErrorPage from '@/pages/GeneralErrorPage';

import styles from './styles.module.scss';

function NavigationLink(label: string, route: Route, icon: IconDefinition) {
    return {
        label,
        route,
        icon,
    } as const;
}

export type NavLink = ReturnType<typeof NavigationLink>;

const navigationLinks: NavLink[] = [
    NavigationLink('Dashboard', UserDashboard, faHome),
    NavigationLink('Exercises', Exercises, faDumbbell),
    NavigationLink('Tracker', UserTracker, faStopwatch),
    NavigationLink('Activities', UserProgress, faList),
];

const AppSiteShell: Component = () => {
    const [isSidebarExpanded, setIsSidebarExpanded] = createSignal(false);

    const toggleSidebar = () => setIsSidebarExpanded(currentState => !currentState);

    return (
        <>
            <Helmet title="PixieFit" />
            <div class={styles.app}>
                <div class="hidden md:block">
                    <CollapsibleSideBar
                        isExpanded={isSidebarExpanded()}
                        links={navigationLinks}
                        onToggle={toggleSidebar}
                        version={{
                            major: 1,
                            minor: 0,
                            patch: 0,
                            stage: 'dev',
                        }}
                    />
                </div>
                <div class={styles.appContainer}>
                    <header class={styles.header}>
                        <div class={styles.headerContainer}>
                            <div class={styles.userImage}>
                                <img src={UserIcon} alt="user icon" />
                            </div>
                        </div>
                    </header>
                    <SuspensefulErrorBoundary
                        error={GeneralErrorPage}
                        loading={LoadingView}
                    >
                        <AuthenticatedView>
                            <main class={styles.main}>
                                <Outlet />
                            </main>
                        </AuthenticatedView>
                    </SuspensefulErrorBoundary>
                </div>
                <div class="block md:hidden">
                    <TabBar links={navigationLinks} />
                </div>
            </div>
        </>
    );
};

export default AppSiteShell;
