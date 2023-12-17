import {type Component} from 'solid-js';

import Helmet from '@/components/Helmet';

const LogoutPage: Component = () => {
    return (
        <>
            <Helmet title="Signed Out" />
            <main class="flex flex-col items-center justify-center">
                <div>
                    <span>
                        You have successfully signed out.
                    </span>
                </div>
            </main>
        </>
    );
};

export default LogoutPage;
