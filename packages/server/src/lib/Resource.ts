import DataTransferObject from '@/lib/DataTransferObject';

export interface ResourceObject {
    data: Record<string, any> | Array<Record<string, any>>;
}

export default class Resource<T extends {}> {
    protected data: DataTransferObject<T> | Array<DataTransferObject<T>>;

    constructor(data: DataTransferObject<T> | Array<DataTransferObject<T>>) {
        this.data = data;
    }

    base(): Record<string, any> {
        return {};
    }

    static make<A extends {}, B extends Resource<A>>(
        this: new (arg: DataTransferObject<A>) => B,
        datum: DataTransferObject<A>,
    ): ResourceObject {
        const instance = new this(datum);

        return {
            data: instance.base(),
        };
    }

    static list<A extends {}, B extends Resource<A>>(
        this: new (arg: DataTransferObject<A>) => B,
        data: Array<DataTransferObject<A>>,
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
