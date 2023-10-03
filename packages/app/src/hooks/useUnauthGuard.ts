import {createRenderEffect} from 'solid-js';

import {UserDashboard} from '@/constants/Routes';
import useRouter from '@/hooks/useRouter';
import {useUser} from '@/stores/auth.store';

export default function useUnauthGuard(): void {
    const router = useRouter();
    const user = useUser();

    createRenderEffect(() => {
        if (user() !== null && !user.loading) {
            router.replace(UserDashboard.href);
        }
    });
}
