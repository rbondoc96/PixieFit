import {MetaProvider} from '@solidjs/meta';
import {Router, useRoutes} from '@solidjs/router';
import {type Component, onMount} from 'solid-js';

import routes from '@/navigation/routes';
import {fetchUser} from '@/stores/auth.store';

const App: Component = () => {
    const Routes = useRoutes(routes);

    onMount(() => fetchUser());

    return (
        <MetaProvider>
            <Router>
                <Routes />
            </Router>
        </MetaProvider>
    );
};

export default App;
