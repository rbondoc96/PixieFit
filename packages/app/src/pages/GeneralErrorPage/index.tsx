import {type Component, createEffect} from 'solid-js';

import UnauthorizedRequestException from '@/exceptions/UnauthorizedRequestException';
import useRouter from '@/hooks/useRouter';

const GeneralErrorPage: Component<{
    error: unknown;
    reset: () => void;
}> = props => {
    const router = useRouter();

    createEffect(
        () => {
            if (props.error instanceof UnauthorizedRequestException) {
                router.replace('/login');
            }
        },
    );

    return (
        <div>
            <h1>General Error Page</h1>
            <span>Please contact support or try again later.</span>
            <div>
                <p>Description of Error:</p>
                <pre>{JSON.stringify(props.error, null, 2)}</pre>
            </div>
        </div>
    );
};

export default GeneralErrorPage;
