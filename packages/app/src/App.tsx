import {MetaProvider} from '@solidjs/meta';
import {Router, useRoutes} from '@solidjs/router';
import {type Component, onCleanup, onMount} from 'solid-js';

import routes from '@/navigation/routes';
import {updateScreenSize} from '@/stores/ui.store';

const App: Component = () => {
    const Routes = useRoutes(routes);

    onMount(() => window.addEventListener('resize', updateScreenSize));
    onCleanup(() => window.removeEventListener('resize', updateScreenSize));

    return (
        <MetaProvider>
            <Router>
                <Routes />
            </Router>
        </MetaProvider>
    );
};

export default App;
