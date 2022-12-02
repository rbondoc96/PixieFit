export default class DataTransferObject {
    private readonly data: Record<string, any>;

    constructor(data: Record<string, any>) {
        for (const [key, value] of Object.entries(data)) {
            (this as Record<string, any> & DataTransferObject)[key] = value;
        }
        this.data = data;
    }

    toJSON(): Record<string, any> {
        return this.data;
    }

    toObject(): Record<string, any> {
        return this.data;
    }
}
