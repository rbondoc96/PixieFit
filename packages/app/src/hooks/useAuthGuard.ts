import {createRenderEffect, onMount} from 'solid-js';

import {Login} from '@/constants/Routes';
import useRouter from '@/hooks/useRouter';
import authStore from '@/stores/auth.store';

export default function useAuthGuard(): void {
    const router = useRouter();

    onMount(() => {
        if (authStore.user() === null) {
            router.replace(Login.fullPath);
        }
    });

    createRenderEffect(() => {
        if (authStore.user() === null) {
            router.replace(Login.fullPath);
        }
    });
}
