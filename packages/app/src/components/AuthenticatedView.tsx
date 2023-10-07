import {createRenderEffect, type ParentComponent} from 'solid-js';

import {Login} from '@/constants/Routes';
import useRouter from '@/hooks/useRouter';
import {useUser} from '@/stores/auth.store';

const AuthenticatedView: ParentComponent = props => {
    const router = useRouter();
    const user = useUser();

    createRenderEffect(() => {
        if (user() === null && !user.loading) {
            router.replace(Login.href);
        }
    });

    return (
        <>
            {props.children}
        </>
    );
};

export default AuthenticatedView;
