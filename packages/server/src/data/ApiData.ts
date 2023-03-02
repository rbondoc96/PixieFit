interface ApiDataResponse<T extends {}, D = Partial<T>> {
    /**
     * Will always be true
     */
    success: true;
    /**
     * Data to be returned to the client
     */
    data: Record<string, D | D[]> | null;
}

abstract class ApiData<T extends {} = {}> {
    protected abstract readonly data: T | T[] | null;
    protected abstract readonly dataKey: string | null;

    protected abstract toJSON(): Partial<T> | Partial<T>[];

    public toJSONResponse(): ApiDataResponse<T> {
        if (this.dataKey !== null) {
            return {
                success: true,
                data: {
                    [this.dataKey]: this.toJSON(),
                },
            };
        }

        return {
            success: true,
            data: null,
        };
    }
}

export default ApiData;
