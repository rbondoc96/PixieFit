import {Outlet} from '@solidjs/router';
import {type Component, ErrorBoundary} from 'solid-js';

import Helmet from '@/components/Helmet';
import Logo from '@/components/Logo';
import TabNavigationBar from '@/components/TabNavigationBar';
import useAuthGuard from '@/hooks/useAuthGuard';
import GeneralErrorPage from '@/pages/GeneralErrorPage';

import styles from './styles.module.scss';

const AppSiteShell: Component = () => {
    useAuthGuard();

    return (
        <>
            <Helmet title="PixieFit" />
            <div class={styles.app}>
                <header class={styles.header}>
                    <Logo />
                </header>
                <main class={styles.main}>
                    <ErrorBoundary fallback={GeneralErrorPage}>
                        <Outlet />
                    </ErrorBoundary>
                </main>
                <TabNavigationBar />
            </div>
        </>
    );
};

export default AppSiteShell;
