import RequestException from '@/exceptions/RequestException';

export class ModelNotFoundException extends RequestException {
    public override readonly name: string = 'ModelNotFoundException';

    constructor(modelName: string, key: string, value: unknown) {
        super(`${modelName} with (${key}, ${value}) does not exist.`, 404);
    }
}

export default ModelNotFoundException;
