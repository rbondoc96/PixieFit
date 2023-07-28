import {type Component} from 'solid-js';

import {useAuthenticatedUser} from '@/stores/auth.store';

import styles from './styles.module.scss';

const DashboardPage: Component = () => {
    const user = useAuthenticatedUser();

    return (
        <section class={styles.dashboardPage}>
            <h1>Welcome, {user.name.full}</h1>
        </section>
    );
};

export default DashboardPage;
