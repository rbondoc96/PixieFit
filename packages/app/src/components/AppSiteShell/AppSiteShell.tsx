import {
    faDumbbell,
    faHome,
    faList,
    faStopwatch,
    type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import {Outlet} from '@solidjs/router';
import {type Component, createSignal} from 'solid-js';

import AppSiteHeader from '@/components/AppSiteShell/AppSiteHeader';
import AuthenticatedView from '@/components/AuthenticatedView';
import Helmet from '@/components/Helmet';
import LoadingView from '@/components/LoadingView';
import CollapsibleSideBar from '@/components/navigation/CollapsibleSideBar';
import TabBar from '@/components/navigation/TabBar';
import SuspensefulErrorBoundary from '@/components/SuspensefulErrorBoundary';
import {
    ExerciseHomePage,
    UserDashboard,
    UserProgress,
    UserTracker,
} from '@/constants/Routes';
import {type Route} from '@/lib/Route';
import GeneralErrorPage from '@/pages/GeneralErrorPage';

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
    NavigationLink('Exercises', ExerciseHomePage, faDumbbell),
    NavigationLink('Tracker', UserTracker, faStopwatch),
    NavigationLink('Activities', UserProgress, faList),
];

const AppSiteShell: Component = () => {
    const [isSidebarExpanded, setIsSidebarExpanded] = createSignal(true);

    const toggleSidebar = () => setIsSidebarExpanded(currentState => !currentState);

    return (
        <>
            <Helmet title="PixieFit" />
            <div class="flex flex-col h-screen md:flex-row">
                <div>
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
                <div class="flex-grow flex flex-col overflow-y-auto">
                    <header class="flex px-6 py-4 shadow-md">
                        <AppSiteHeader
                            isSidebarExpanded={isSidebarExpanded()}
                            onSidebarToggle={toggleSidebar}
                        />
                    </header>
                    <SuspensefulErrorBoundary
                        error={GeneralErrorPage}
                        loading={LoadingView}
                    >
                        <AuthenticatedView>
                            <main class="relative flex-1 flex flex-col overflow-y-auto">
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
