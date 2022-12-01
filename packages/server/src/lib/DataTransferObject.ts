export default class DataTransferObject<T extends {}> {
    private readonly data: T;

    constructor(data: T) {
        for (const [key, value] of Object.entries(data)) {
            (this as Record<string, any> & DataTransferObject<T>)[key] = value;
        }
        this.data = data;
    }

    toJSON(): T {
        return this.data;
    }

    toObject(): T {
        return this.data;
    }
}
