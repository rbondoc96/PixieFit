import {type Component} from 'solid-js';

import Button from '@/components/Button';
import {logout} from '@/stores/auth.store';

import styles from './styles.module.scss';

const SettingsPage: Component = () => {
    return (
        <main class={styles.main}>
            <h1>Settings Page</h1>
            <Button onClick={logout} label="Sign Out" />
        </main>
    );
};

export default SettingsPage;
