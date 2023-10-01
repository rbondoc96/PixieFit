import {Outlet} from '@solidjs/router';
import {type Component, ErrorBoundary, Suspense} from 'solid-js';

import UserIcon from '@/assets/images/user.png';
import Helmet from '@/components/Helmet';
import LoadingView from '@/components/LoadingView';
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
                    <div class={styles.headerContainer}>
                        <div />
                        <Logo />
                        <div class={styles.userImage}>
                            <img src={UserIcon} alt="user icon" />
                        </div>
                    </div>
                </header>
                <main class={styles.main}>
                    <Suspense fallback={<LoadingView />}>
                        <ErrorBoundary fallback={GeneralErrorPage}>
                            <Outlet />
                        </ErrorBoundary>
                    </Suspense>
                </main>
                <TabNavigationBar />
            </div>
        </>
    );
};

export default AppSiteShell;
