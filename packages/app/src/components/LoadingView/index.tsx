import {type Component} from 'solid-js';

import Spinner from '@/components/Spinner';

import styles from './styles.module.scss';

const LoadingView: Component = () => {
    return (
        <div class={styles.loadingView}>
            <Spinner color="primary" size="xl" />
            <span class={styles.loadingViewLabel}>
                Loading...
            </span>
        </div>
    );
};

export default LoadingView;
