import {type Component} from 'solid-js';

import {useAuthenticatedUser} from '@/stores/auth.store';

import styles from './styles.module.scss';

const DashboardPage: Component = () => {
    const user = useAuthenticatedUser();

    return (
        <div class={styles.section}>
            <div class={styles.userInfo}>
                <div class={styles.userGreeting}>
                    <span class={styles.userWelcome}>Welcome back,</span>
                    <span class={styles.userName}>{user.name.full}</span>
                </div>
            </div>
            <section class={styles.activity}>
                <h2>Today&apos;s Activity</h2>
            </section>
        </div>
    );
};

export default DashboardPage;
