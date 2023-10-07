import {type Component} from 'solid-js';

import Spinner from '@/components/Spinner';

import styles from './styles.module.scss';

const LoadingView: Component = () => {
    return (
        <div class="relative flex-1">
            <div class="absolute inset-0 flex flex-col justify-center items-center gap-y-2">
                <Spinner color="primary" size="xl" />
                <span class={styles.loadingViewLabel}>
                    Loading...
                </span>
            </div>
        </div>
    );
};

export default LoadingView;
