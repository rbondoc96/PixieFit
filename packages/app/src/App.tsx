import {MetaProvider} from '@solidjs/meta';
import {Router, useRoutes} from '@solidjs/router';
import {type Component} from 'solid-js';

import routes from '@/navigation/routes';

const App: Component = () => {
    const Routes = useRoutes(routes);

    return (
        <MetaProvider>
            <Router>
                <Routes />
            </Router>
        </MetaProvider>
    );
};

export default App;
