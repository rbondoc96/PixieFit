import {createRenderEffect, onMount} from 'solid-js';

import {UserDashboard} from '@/constants/Routes';
import useRouter from '@/hooks/useRouter';
import authStore from '@/stores/auth.store';

export default function useUnauthGuard(): void {
    const router = useRouter();

    onMount(() => {
        if (authStore.user() !== null) {
            router.replace(UserDashboard.fullPath);
        }
    });

    createRenderEffect(() => {
        if (authStore.user() !== null) {
            router.replace(UserDashboard.fullPath);
        }
    });
}
