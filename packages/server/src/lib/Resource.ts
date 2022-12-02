import DataTransferObject from '@/lib/DataTransferObject';

export interface ResourceObject {
    data: Record<string, any> | Array<Record<string, any>>;
}

export default class Resource<D extends DataTransferObject> {
    protected data: D;

    constructor(data: D) {
        this.data = data;
    }

    base(): Record<string, any> {
        return {};
    }

    static make<D extends DataTransferObject, R extends Resource<D>>(
        this: new (arg: D) => R,
        datum: D,
    ): ResourceObject {
        const instance = new this(datum);

        return {
            data: instance.base(),
        };
    }

    static list<D extends DataTransferObject, R extends Resource<D>>(
        this: new (arg: D) => R,
        data: D[],
    ): ResourceObject {
        const resources = [];

        for (const datum of data) {
            const instance = new this(datum);
            resources.push(instance.base());
        }

        return {
            data: resources,
        };
    }
}
