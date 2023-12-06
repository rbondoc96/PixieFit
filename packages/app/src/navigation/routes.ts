import {type RouteDefinition} from '@solidjs/router';
import {lazy} from 'solid-js';

import AppSiteShell from '@/components/AppSiteShell';
import * as Routes from '@/constants/Routes';
import Error404Page from '@/pages/Error404Page';
import LandingPage from '@/pages/LandingPage';
import LoginPage from '@/pages/LoginPage';
import LogoutPage from '@/pages/LogoutPage';
import RegisterPage from '@/pages/RegisterPage';

export default [
    {
        path: Routes.Landing.path,
        component: LandingPage,
    },
    {
        path: Routes.Login.path,
        component: LoginPage,
    },
    {
        path: Routes.Logout.path,
        component: LogoutPage,
    },
    {
        path: Routes.Register.path,
        component: RegisterPage,
    },
    {
        path: '/app',
        component: AppSiteShell,
        children: [
            {
                path: Routes.UserDashboard.path,
                component: lazy(() => import('@/pages/app/DashboardPage')),
            },
            {
                path: Routes.ExerciseHomePage.path,
                component: lazy(() => import('../pages/app/exercises/ExerciseHomePage')),
            },
            {
                path: Routes.UserTracker.path,
                component: lazy(() => import('@/pages/app/TrackerPage')),
            },
            {
                path: Routes.UserProgress.path,
                component: lazy(() => import('@/pages/app/ProgressPage')),
            },
            {
                path: Routes.UserSettings.path,
                component: lazy(() => import('@/pages/app/SettingsPage')),
            },
            {
                path: '/*',
                component: Error404Page,
            },
        ],
    },
    {
        path: '*',
        component: Error404Page,
    },
] satisfies RouteDefinition[];
