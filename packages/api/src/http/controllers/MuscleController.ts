import Controller from '@pxeeio/express-async-controller';

import * as WGERService from '@/services/WGER/api/WGERService';

type ReadRequest = {
    params: {
        id: string;
    };
};

export default Controller<{
    index: undefined;
    read: ReadRequest;
}>({
    index: async (request, response) => {
        const muscles = await WGERService.listMuscles();

        response.ok().jsonResponse({
            data: muscles,
        });
    },
    read: async (request, response) => {
        const {id} = request.params;

        if (Number.isNaN(Number(id))) {
            throw new Error('Invalid Muscle ID');
        }

        // TODO: Better error handling
        const muscle = await WGERService.readMuscle(Number(id));

        response.ok().jsonResponse({
            data: muscle,
        });
    },
});
