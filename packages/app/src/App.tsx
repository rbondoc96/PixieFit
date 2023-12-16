import {MetaProvider} from '@solidjs/meta';
import {Router, useRoutes} from '@solidjs/router';
import {type Component, onCleanup, onMount} from 'solid-js';

import routes from '@/navigation/routes';
import {useOnWindowResize} from '@/stores/ui.store';

const App: Component = () => {
    const Routes = useRoutes(routes);
    const onWindowResize = useOnWindowResize();

    onMount(() => window.addEventListener('resize', onWindowResize));
    onCleanup(() => window.removeEventListener('resize', onWindowResize));

    return (
        <MetaProvider>
            <Router>
                <Routes />
            </Router>
        </MetaProvider>
    );
};

export default App;
