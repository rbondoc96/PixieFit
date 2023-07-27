import {useNavigate} from '@solidjs/router';

import {Route} from '@/lib/Route';

type Router = {
    push: (pathOrRoute: string | Route) => void;
    replace: (pathOrRoute: string | Route) => void;
};

export default function useRouter(): Router {
    const navigate = useNavigate();

    const push = (pathOrRoute: string | Route): void => {
        if (pathOrRoute instanceof Route) {
            navigate(pathOrRoute.fullPath);
        } else {
            navigate(pathOrRoute);
        }
    };

    const replace = (pathOrRoute: string | Route): void => {
        if (pathOrRoute instanceof Route) {
            navigate(pathOrRoute.fullPath, {
                replace: true,
            });
        } else {
            navigate(pathOrRoute, {
                replace: true,
            });
        }
    };

    return {
        push,
        replace,
    };
}
